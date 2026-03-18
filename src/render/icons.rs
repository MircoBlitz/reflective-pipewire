/// Full SVG content for an icon.
pub type IconData = &'static str;

macro_rules! icon {
    ($file:expr) => {
        include_str!(concat!("../../assets/icons/", $file, ".svg"))
    };
}

pub fn get_active(name: &str) -> IconData {
    match name {
        "mic" => icon!("mic"),
        "headset" => icon!("headset"),
        "headphones" => icon!("headphones"),
        "stereo" => icon!("stereo"),
        "jack" => icon!("jack"),
        "dac" => icon!("dac"),
        "apps" => icon!("apps"),
        "discord" => icon!("discord"),
        "obs" => icon!("obs"),
        "twitch" => icon!("twitch"),
        "speaker" => icon!("speaker"),
        "mixer" => icon!("mixer"),
        _ => icon!("mic"),
    }
}

pub fn get_deactivated(name: &str) -> IconData {
    match name {
        "mic" => icon!("mic_deactivated"),
        "headset" => icon!("headset_deactivated"),
        "headphones" => icon!("headphones_deactivated"),
        "stereo" => icon!("stereo_deactivated"),
        "jack" => icon!("jack_deactivated"),
        "dac" => icon!("dac_deactivated"),
        "apps" => icon!("apps_deactivated"),
        "discord" => icon!("discord_deactivated"),
        "obs" => icon!("obs_deactivated"),
        "twitch" => icon!("twitch_deactivated"),
        "speaker" => icon!("speaker_deactivated"),
        "mixer" => icon!("mixer_deactivated"),
        _ => icon!("mic_deactivated"),
    }
}

pub fn get(name: &str, active: bool) -> IconData {
    if active {
        get_active(name)
    } else {
        get_deactivated(name)
    }
}

pub const AVAILABLE: &[(&str, &str)] = &[
    ("mic", "Mikrofon"),
    ("headset", "Headset"),
    ("headphones", "Kopfhörer"),
    ("stereo", "Anlage"),
    ("jack", "3.5mm Jack"),
    ("dac", "Externes DAC"),
    ("apps", "Anwendungen"),
    ("discord", "Discord"),
    ("obs", "OBS"),
    ("twitch", "Twitch"),
    ("speaker", "Lautsprecher"),
    ("mixer", "Mixer"),
];