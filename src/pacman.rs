use embedded_graphics::{drawable::Pixel, pixelcolor::Rgb565, prelude::*};

pub struct Pacman {
    pub x: i32,
    pub y: i32,
    mouth_open: bool,
}

impl Pacman {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            mouth_open: true,
        }
    }

    pub fn update(&mut self) {
        self.x += 1;
        self.mouth_open = !self.mouth_open;

        // Quand Pac-Man est complètement sorti à droite,
        // on le remet complètement à gauche.
        if self.x >= 64 {
            self.x = -8;
        }
    }

    pub fn draw<P>(&self, display: &mut P)
    where
        P: Drawing<Rgb565>,
    {
        let yellow = Rgb565::from((255u8, 220u8, 0u8));
        let black = Rgb565::from((0u8, 0u8, 0u8));

        // Corps de Pac-Man : 8 x 8 pixels
        //
        //  . Y Y Y Y .
        //  Y Y Y Y Y Y
        //  Y Y Y Y Y Y
        //  Y Y Y Y Y Y
        //  Y Y Y Y Y Y
        //  Y Y Y Y Y Y
        //  . Y Y Y Y .
        //  . . . . . .
        //

        for py in 0..8 {
            for px in 0..8 {
                let draw_pixel = if self.mouth_open {
                    // Bouche ouverte vers la droite
                    !((px >= 5 && py <= 2) || (px >= 5 && py >= 5))
                } else {
                    true
                };

                if draw_pixel {
                    let x = self.x + px;
                    let y = self.y + py;

                    if x >= 0 && x < 64 && y >= 0 && y < 32 {
                        display.draw(core::iter::once(Pixel(
                            UnsignedCoord::new(x as u32, y as u32),
                            yellow,
                        )));
                    }
                }
            }
        }

        // Œil
        let eye_x = self.x + 5;
        let eye_y = self.y + 1;

        if eye_x >= 0 && eye_x < 64 && eye_y >= 0 && eye_y < 32 {
            display.draw(core::iter::once(Pixel(
                UnsignedCoord::new(eye_x as u32, eye_y as u32),
                black,
            )));
        }
    }
}
