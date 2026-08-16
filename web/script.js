const CATEGORIES = [
  {
    name: "Tests",
    items: [
      { id: "red_test", label: "red_test", sub: "aplat rouge plein", color: "#ff3b3b" },
      { id: "gradient_test", label: "gradient_test", sub: "rampe rouge / vert", color: "#35e07a" },
      {
        id: "random_test",
        label: "random_test",
        sub: "bruit multicolore, image fixe",
        color: "#b06bff",
      },
      {
        id: "random_loop",
        label: "random_loop",
        sub: "bruit multicolore, boucle 5s",
        color: "#3b8bff",
      },
    ],
  },
  {
    name: "Drapeaux",
    items: [{ id: "flag_basque", label: "Ikurriña", sub: "drapeau basque", color: "#e30613" }],
  },
];

// ---------- construire la grille de la maquette (64x32) ----------
const matrixEl = document.getElementById("matrix");
const cells = [];
for (let i = 0; i < 64 * 32; i++) {
  const c = document.createElement("div");
  c.className = "cell";
  matrixEl.appendChild(c);
  cells.push(c);
}

function paintMatrix(colorHex) {
  cells.forEach((c) => {
    if (!colorHex) {
      c.style.background = "#1b1d22";
      return;
    }
    const flicker = Math.random() > 0.15;
    c.style.background = flicker ? colorHex : "#1b1d22";
  });
}
paintMatrix(null);

// ---------- construire les catégories + cartes ----------
const categoriesEl = document.getElementById("categories");

CATEGORIES.forEach((cat, catIndex) => {
  const details = document.createElement("details");
  details.className = "category";
  if (catIndex === 0) details.open = true; // première catégorie ouverte par défaut

  const summary = document.createElement("summary");
  summary.innerHTML = `<span>${cat.name}</span><span class="count">${cat.items.length}</span>`;
  details.appendChild(summary);

  const grid = document.createElement("div");
  grid.className = "patterns";

  cat.items.forEach((p) => {
    const btn = document.createElement("button");
    btn.className = "pattern-card";
    btn.style.setProperty("--accent", p.color);
    btn.disabled = true;
    btn.dataset.id = p.id;
    btn.innerHTML = `
      <div class="swatch" style="background:${p.color}"></div>
      <div class="pattern-name">${p.label}</div>
      <div class="pattern-key mono">${p.sub}</div>
    `;
    btn.addEventListener("click", () => sendCommand(p));
    grid.appendChild(btn);
  });

  details.appendChild(grid);
  categoriesEl.appendChild(details);
});

// ---------- état série ----------
let port = null;
let writer = null;
let reader = null;
let keepReading = false;

const statusEl = document.getElementById("status");
const statusTextEl = document.getElementById("status-text");
const connectBtn = document.getElementById("connect-btn");
const consoleEl = document.getElementById("console");
const heroModeEl = document.getElementById("hero-mode");

function log(text, cls) {
  const line = document.createElement("div");
  line.className = "line" + (cls ? " " + cls : "");
  line.textContent = text;
  consoleEl.appendChild(line);
  consoleEl.scrollTop = consoleEl.scrollHeight;
}

function setConnected(isConnected, label) {
  statusEl.classList.toggle("on", isConnected);
  statusTextEl.textContent = label || (isConnected ? "connecté" : "non connecté");
  connectBtn.textContent = isConnected ? "Déconnecter" : "Connecter le Pico";
  document.querySelectorAll(".pattern-card").forEach((b) => (b.disabled = !isConnected));
  if (!isConnected) {
    paintMatrix(null);
    heroModeEl.textContent = "en attente";
    document.querySelectorAll(".pattern-card").forEach((b) => b.classList.remove("active"));
  }
}

async function connect() {
  if (!("serial" in navigator)) {
    setConnected(false, "Web Serial non supporté");
    statusEl.classList.add("err");
    log("Ce navigateur ne supporte pas Web Serial. Utilisez Chrome ou Edge.", "sys");
    return;
  }
  try {
    port = await navigator.serial.requestPort();
    await port.open({ baudRate: 115200 });
    log("Port ouvert.", "sys");
    setConnected(true, "connecté");
    statusEl.classList.remove("err");

    writer = port.writable.getWriter();
    keepReading = true;
    readLoop();
  } catch (e) {
    log("Connexion annulée ou échouée: " + e, "sys");
  }
}

async function disconnect() {
  keepReading = false;
  try {
    if (reader) {
      await reader.cancel();
    }
    if (writer) {
      writer.releaseLock();
    }
    if (port) {
      await port.close();
    }
  } catch (e) {
    log("Erreur à la déconnexion: " + e, "sys");
  }
  port = null;
  writer = null;
  reader = null;
  setConnected(false);
  log("Déconnecté.", "sys");
}

async function readLoop() {
  const decoder = new TextDecoderStream();
  port.readable.pipeTo(decoder.writable);
  reader = decoder.readable.getReader();
  let buf = "";
  try {
    while (keepReading) {
      const { value, done } = await reader.read();
      if (done) break;
      if (value) {
        buf += value;
        let idx;
        while ((idx = buf.indexOf("\n")) >= 0) {
          const line = buf.slice(0, idx).replace(/\r$/, "");
          if (line.length) log(line);
          buf = buf.slice(idx + 1);
        }
      }
    }
  } catch (e) {
    log("Lecture interrompue: " + e, "sys");
  } finally {
    reader.releaseLock();
  }
}

async function sendCommand(pattern) {
  if (!writer) return;
  const payload = pattern.id + "\n";
  await writer.write(new TextEncoder().encode(payload));
  log("> " + pattern.id, "tx");
  document
    .querySelectorAll(".pattern-card")
    .forEach((b) => b.classList.toggle("active", b.dataset.id === pattern.id));
  heroModeEl.textContent = pattern.label.toLowerCase();
  paintMatrix(pattern.color);
}

connectBtn.addEventListener("click", () => {
  if (port) disconnect();
  else connect();
});
