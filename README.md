# Reflective PipeWire

A reactive PipeWire audio suite plugin for [OpenDeck](https://github.com/amansprojects/opendeck) (Stream Deck on Linux). All buttons sync their state in real-time — mute your mic via a pedal and every button across all your Stream Decks reflects the change instantly.

## Features

- **Real-time state sync** across all buttons and devices via `pactl subscribe`
- **4 action types**: Mute Toggle, Volume Knob, Volume Display, Volume Button
- **12 built-in icons**: Mic, Headset, Headphones, Stereo, 3.5mm Jack, External DAC, Applications, Discord, OBS, Twitch, Speaker, Mixer
- **Per-button customization**: icon colors (active/muted), background colors (active/muted)
- **Device-aware**: all buttons watching the same device update together
- Works with any PipeWire/PulseAudio source or sink

## Requirements

- Linux with PipeWire (and PulseAudio compatibility layer)
- [OpenDeck](https://github.com/amansprojects/opendeck) installed
- `wpctl` and `pactl` available in PATH

## Installation

1. Download the latest `com.mircoblitz.reflective-pipewire.zip` from [Releases](https://github.com/MircoBlitz/reflective-pipewire/releases)
2. In OpenDeck, go to **Plugins → Install from file**
3. Select the downloaded `.zip` file
4. The plugin appears under **RefPipe Audio** in the action sidebar

## Actions

### Mute Toggle (Keypad)

Toggles mute on any source or sink. The icon color switches between active (default: green) and muted (default: red).

**Settings:**
- **Device** — select any source/sink or use the system default
- **Icon** — choose from 12 built-in icons
- **Icon Colors** — active and muted color (applied to the icon)
- **Background Colors** — active and muted color (default: black/black)

### Volume Knob (Encoder)

For Stream Deck+ dials. Rotate to change volume, press to toggle mute. Shows a volume bar and percentage.

**Settings:**
- **Device** / **Icon** — same as Mute Toggle
- **Step per Tick** — volume change per encoder tick (default: 5%)
- **Icon Colors** / **Background Colors** / **Bar Color**

### Volume Display (Keypad)

Read-only button that shows the current volume as a bar and percentage. Updates in real-time when volume changes from any source.

**Settings:**
- **Device** / **Icon** — same as Mute Toggle
- **Icon Colors** / **Background Colors** / **Bar Color**

### Volume Button (Keypad)

Press to adjust volume. Three modes:

| Mode | Behavior |
|------|----------|
| **Volume Up** | Increases volume by step % |
| **Volume Down** | Decreases volume by step % |
| **Set Fixed Volume** | Sets volume to an exact percentage |

**Settings:**
- **Device** / **Icon** / **Mode** / **Step** / **Fixed Value**
- **Reflect mute state** — checkbox to enable icon color changes on mute (off by default)
- **Icon Colors** / **Background Colors**

## Building from source

```bash
git clone https://github.com/MircoBlitz/reflective-pipewire.git
cd reflective-pipewire
cargo build --release
```

To package as an installable plugin:

```bash
mkdir -p dist/com.mircoblitz.reflective-pipewire.sdPlugin/x86_64-unknown-linux-gnu
cp target/release/reflective-pipewire dist/com.mircoblitz.reflective-pipewire.sdPlugin/x86_64-unknown-linux-gnu/
cp manifest.json dist/com.mircoblitz.reflective-pipewire.sdPlugin/
cp -r propertyInspector icons dist/com.mircoblitz.reflective-pipewire.sdPlugin/
cd dist && zip -r com.mircoblitz.reflective-pipewire.zip com.mircoblitz.reflective-pipewire.sdPlugin
```

Then install the `.zip` via OpenDeck.

## How it works

The plugin spawns a background `pactl subscribe` process that monitors all audio events. When any source or sink changes state, every visible button watching that device re-renders with the current volume and mute state. Actions that modify a device (toggle mute, change volume) trigger an immediate sync of all buttons for that device across all action types.

## License

MIT
