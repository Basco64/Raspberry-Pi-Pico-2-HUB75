use rp235x_hal::rom_data;

const FLASH_TARGET_OFFSET: u32 = 2 * 1024 * 1024 - 4096;
const FLASH_SECTOR_SIZE: usize = 4096;
const XIP_BASE: usize = 0x1000_0000;

const MAGIC: u32 = 0x5049_4357; // "PICW" -- marque que ce secteur contient bien nos données
const MAX_TEXT_LEN: usize = 60;

/// Sauvegarde le texte en flash (persiste après coupure d'alimentation).
pub fn save_text(text: &str) {
    let bytes = text.as_bytes();
    let len = bytes.len().min(MAX_TEXT_LEN);

    let mut buf = [0u8; FLASH_SECTOR_SIZE];
    buf[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    buf[4..8].copy_from_slice(&(len as u32).to_le_bytes());
    buf[8..8 + len].copy_from_slice(&bytes[..len]);

    critical_section::with(|_| unsafe {
        rom_data::flash_range_erase(
            FLASH_TARGET_OFFSET,
            FLASH_SECTOR_SIZE as usize,
            1 << 16,
            0xD8,
        );
        rom_data::flash_range_program(
            FLASH_TARGET_OFFSET,
            buf.as_ptr(),
            FLASH_SECTOR_SIZE as usize,
        );
        rom_data::flash_flush_cache();
    });
}

pub fn load_text(out: &mut heapless::String<64>) -> bool {
    let ptr = (XIP_BASE + FLASH_TARGET_OFFSET as usize) as *const u8;

    let magic = unsafe { core::ptr::read_volatile(ptr as *const u32) };
    if magic != MAGIC {
        return false;
    }

    let len = unsafe { core::ptr::read_volatile(ptr.add(4) as *const u32) } as usize;
    if len == 0 || len > MAX_TEXT_LEN {
        return false;
    }

    let slice = unsafe { core::slice::from_raw_parts(ptr.add(8), len) };
    match core::str::from_utf8(slice) {
        Ok(s) => {
            out.clear();
            let _ = out.push_str(s);
            true
        }
        Err(_) => false,
    }
}
