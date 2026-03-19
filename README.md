# Reflective PipeWire

A reflective PipeWire audio suite plugin for [OpenDeck](https://github.com/nekename/OpenDeck) (Stream Deck on Linux). All buttons sync their state in real-time - mute your mic via a pedal and every button across all your Stream Decks reflects the change instantly.

## Features

- **Real-time state sync** across all buttons and devices via `pactl subscribe`
- **5 action types**: Mute Toggle, Volume Knob, Volume Display, Volume Up, Volume Down
- **18 built-in icons**: Mic, Headset, Headphones, Stereo, 3.5mm Jack, External DAC, Applications, Discord, OBS, Twitch, Speaker, Mixer, Plus, Minus, Chevron Up, Chevron Down, Volume Up, Volume Down
- **Per-button customization**: icon colors (active/muted), background colors (active/muted)
- **Device-aware**: all buttons watching the same device update together
- Works with any PipeWire/PulseAudio source or sink

## Requirements

- Linux with PipeWire (and PulseAudio compatibility layer)
- [OpenDeck](https://github.com/nekename/OpenDeck) installed
- `wpctl` and `pactl` available in PATH

## Installation

1. Download the latest `de.mircoblitz.reflective-pipewire.zip` from [Releases](https://github.com/MircoBlitz/reflective-pipewire/releases)
2. In OpenDeck, go to **Plugins → Install from file**
3. Select the downloaded `.zip` file
4. The plugin appears under **RefPipe Audio** in the action sidebar

## Actions

### Mute Toggle (Keypad)

Toggles mute on any source or sink. The icon color switches between active (default: green) and muted (default: red).

**Settings:**
- **Device** - select any source/sink or use the system default
- **Icon** - choose from 12 built-in icons
- **Icon Colors** - active and muted color (applied to the icon)
- **Background Colors** - active and muted color (default: black/black)

### Volume Knob (Encoder)

For Stream Deck+ dials. Rotate to change volume, press to toggle mute. Shows a volume bar and percentage.

> **Note:** Volume can be raised up to 150%. Values above 100% are only effective if **"Allow volume to exceed 100%"** (or equivalent) is enabled in your system sound settings - otherwise the system silently caps at 100%.

**Settings:**
- **Device** / **Icon** - same as Mute Toggle
- **Step per Tick** - volume change per encoder tick (default: 5%)
- **Icon Colors** / **Background Colors** / **Bar Color**

### Volume Display (Keypad)

Read-only button that shows the current volume as a bar and percentage. Updates in real-time when volume changes from any source.

**Settings:**
- **Device** / **Icon** - same as Mute Toggle
- **Icon Colors** / **Background Colors** / **Bar Color**

### Volume Up (Keypad)

Press to increase volume by a configurable step. Maximum is 150%.

> **Note:** Values above 100% are only effective if **"Allow volume to exceed 100%"** (or equivalent) is enabled in your system sound settings - otherwise the system silently caps at 100%.

**Settings:**
- **Device** / **Icon** / **Step**
- **Reflect mute state** - checkbox to enable icon color changes on mute (off by default)
- **Icon Colors** / **Background Colors**

### Volume Down (Keypad)

Press to decrease volume by a configurable step.

**Settings:**
- **Device** / **Icon** / **Step**
- **Reflect mute state** - checkbox to enable icon color changes on mute (off by default)
- **Icon Colors** / **Background Colors**

## Building from source

```bash
git clone https://github.com/MircoBlitz/reflective-pipewire.git
cd reflective-pipewire
cargo build --release
```

To package as an installable plugin:

```bash
mkdir -p dist/de.mircoblitz.reflective-pipewire.sdPlugin/x86_64-unknown-linux-gnu
cp target/release/reflective-pipewire dist/de.mircoblitz.reflective-pipewire.sdPlugin/x86_64-unknown-linux-gnu/
cp manifest.json dist/de.mircoblitz.reflective-pipewire.sdPlugin/
cp -r propertyInspector icons dist/de.mircoblitz.reflective-pipewire.sdPlugin/
cd dist && zip -r de.mircoblitz.reflective-pipewire.zip de.mircoblitz.reflective-pipewire.sdPlugin
```

Then install the `.zip` via OpenDeck.

## How it works

The plugin spawns a background `pactl subscribe` process that monitors all audio events. When any source or sink changes state, every visible button watching that device re-renders with the current volume and mute state. Actions that modify a device (toggle mute, change volume) trigger an immediate sync of all buttons for that device across all action types.

## Known Issues

- **Buttons turn white when dropping an encoder action (e.g. Volume Knob) onto a keypad slot.** OpenDeck clears all button images internally when this happens. Buttons recover on the next interaction - an audio event, button press, or adding another button.

## Acknowledgements

Thanks to the [OpenDeck](https://github.com/nekename/OpenDeck) team for building an open Stream Deck runtime for Linux - this plugin wouldn't exist without it.

## License

MIT
