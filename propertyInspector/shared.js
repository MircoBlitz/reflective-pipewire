/**
 * Shared Property Inspector logic for reflective-pipewire.
 *
 * Usage in each PI HTML:
 *   1. Include this script
 *   2. Define window.piFields = { key: { type, default }, ... }
 *      Types: "color", "select", "text", "number", "checkbox"
 *   3. DOM elements: id="${key}" for most, id="${key}_picker" + id="${key}" for color
 */

let ws;
let piUuid;
let settings = {};

const connectElgatoStreamDeckSocket = (port, uuid, event, info) => {
  piUuid = uuid;
  ws = new WebSocket(`ws://localhost:${port}`);

  ws.onopen = () => {
    ws.send(JSON.stringify({ event, uuid }));
    ws.send(JSON.stringify({ event: "getSettings", context: uuid }));
  };

  ws.onmessage = (msg) => {
    const data = JSON.parse(msg.data);
    if (data.event === "didReceiveSettings") {
      settings = data.payload.settings || {};
      loadFields();
    }
    if (data.event === "sendToPropertyInspector") {
      const payload = data.payload || {};
      if (payload.devices) {
        populateDeviceDropdown(payload.devices);
      }
    }
  };

  // Bind change events
  for (const [key, field] of Object.entries(window.piFields || {})) {
    if (field.type === "color") {
      const picker = document.getElementById(`${key}_picker`);
      const text = document.getElementById(key);
      if (picker && text) {
        picker.addEventListener("input", () => { text.value = picker.value; save(); });
        text.addEventListener("change", () => { picker.value = text.value; save(); });
      }
    } else if (field.type === "checkbox") {
      const el = document.getElementById(key);
      if (el) el.addEventListener("change", save);
    } else {
      const el = document.getElementById(key);
      if (el) {
        el.addEventListener("change", save);
        if (el.type === "number") el.addEventListener("input", save);
      }
    }
  }
};

function loadFields() {
  for (const [key, field] of Object.entries(window.piFields || {})) {
    const val = settings[key] ?? field.default;
    if (field.type === "color") {
      const picker = document.getElementById(`${key}_picker`);
      const text = document.getElementById(key);
      if (picker) picker.value = val;
      if (text) text.value = val;
    } else if (field.type === "checkbox") {
      const el = document.getElementById(key);
      if (el) el.checked = !!val;
    } else {
      const el = document.getElementById(key);
      if (el) el.value = val;
    }
  }
}

function save() {
  const payload = {};
  for (const [key, field] of Object.entries(window.piFields || {})) {
    if (field.type === "color") {
      payload[key] = document.getElementById(key)?.value ?? field.default;
    } else if (field.type === "number") {
      payload[key] = parseInt(document.getElementById(key)?.value ?? field.default, 10);
    } else if (field.type === "checkbox") {
      payload[key] = document.getElementById(key)?.checked ?? field.default;
    } else {
      payload[key] = document.getElementById(key)?.value ?? field.default;
    }
  }
  ws.send(JSON.stringify({
    event: "setSettings",
    context: piUuid,
    payload,
  }));
}

function populateDeviceDropdown(devices) {
  const sel = document.getElementById("device_id");
  if (!sel) return;
  const current = sel.value || settings.device_id;
  sel.innerHTML = "";

  // Defaults group
  const defaults = document.createElement("optgroup");
  defaults.label = "Defaults";
  const defSrc = document.createElement("option");
  defSrc.value = "@DEFAULT_AUDIO_SOURCE@";
  defSrc.textContent = "Default Source (Mic)";
  defaults.appendChild(defSrc);
  const defSnk = document.createElement("option");
  defSnk.value = "@DEFAULT_AUDIO_SINK@";
  defSnk.textContent = "Default Sink (Speaker)";
  defaults.appendChild(defSnk);
  sel.appendChild(defaults);

  // Sources group
  const sources = devices.filter(d => d.kind === "source");
  if (sources.length) {
    const grp = document.createElement("optgroup");
    grp.label = "Sources (Input)";
    for (const dev of sources) {
      const opt = document.createElement("option");
      opt.value = dev.id;
      opt.textContent = dev.description || dev.name;
      grp.appendChild(opt);
    }
    sel.appendChild(grp);
  }

  // Sinks group
  const sinks = devices.filter(d => d.kind === "sink");
  if (sinks.length) {
    const grp = document.createElement("optgroup");
    grp.label = "Sinks (Output)";
    for (const dev of sinks) {
      const opt = document.createElement("option");
      opt.value = dev.id;
      opt.textContent = dev.description || dev.name;
      grp.appendChild(opt);
    }
    sel.appendChild(grp);
  }

  sel.value = current;
}
