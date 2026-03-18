// Font Awesome Free 6.7.2
// License: Icons CC BY 4.0, Fonts SIL OFL 1.1, Code MIT
// https://fontawesome.com/license/free

/// Icon data: (viewBox width, viewBox height, SVG path d)
pub type IconData = (u32, u32, &'static str);

macro_rules! fa_icon {
    ($file:expr) => {{
        const SVG: &str = include_str!(concat!("../../assets/fa/", $file, ".svg"));
        parse_icon(SVG)
    }};
}

const fn parse_icon(svg: &str) -> IconData {
    let bytes = svg.as_bytes();

    let (vw, i) = parse_viewbox_dim(bytes, find_viewbox(bytes));
    let (vh, _) = parse_viewbox_dim(bytes, skip_spaces(bytes, i));
    let d_start = find_d_attr(bytes);
    let d_end = find_d_end(bytes, d_start);

    // Safety: we know the source SVGs are valid UTF-8
    let d = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(bytes.as_ptr().add(d_start), d_end - d_start)) };
    (vw, vh, d)
}

const fn find_viewbox(bytes: &[u8]) -> usize {
    let pattern = b"viewBox=\"0 0 ";
    let plen = pattern.len();
    let mut i = 0;
    while i + plen <= bytes.len() {
        let mut ok = true;
        let mut j = 0;
        while j < plen {
            if bytes[i + j] != pattern[j] { ok = false; break; }
            j += 1;
        }
        if ok { return i + plen; }
        i += 1;
    }
    0
}

const fn skip_spaces(bytes: &[u8], mut i: usize) -> usize {
    while i < bytes.len() && bytes[i] == b' ' { i += 1; }
    i
}

const fn parse_viewbox_dim(bytes: &[u8], mut i: usize) -> (u32, usize) {
    let mut n: u32 = 0;
    while i < bytes.len() && bytes[i] >= b'0' && bytes[i] <= b'9' {
        n = n * 10 + (bytes[i] - b'0') as u32;
        i += 1;
    }
    (n, i + 1)
}

const fn find_d_attr(bytes: &[u8]) -> usize {
    let pattern = b"d=\"";
    let plen = pattern.len();
    let mut i = 0;
    while i + plen <= bytes.len() {
        let mut ok = true;
        let mut j = 0;
        while j < plen {
            if bytes[i + j] != pattern[j] { ok = false; break; }
            j += 1;
        }
        if ok { return i + plen; }
        i += 1;
    }
    0
}

const fn find_d_end(bytes: &[u8], start: usize) -> usize {
    let mut i = start;
    while i < bytes.len() && bytes[i] != b'"' { i += 1; }
    i
}

pub fn get_active(name: &str) -> IconData {
    match name {
        "mic" => fa_icon!("mic"),
        "headset" => fa_icon!("headset"),
        "headphones" => fa_icon!("headphones"),
        "stereo" => fa_icon!("stereo"),
        "jack" => fa_icon!("jack"),
        "dac" => fa_icon!("dac"),
        "apps" => fa_icon!("apps"),
        "discord" => fa_icon!("discord"),
        "obs" => fa_icon!("obs"),
        "twitch" => fa_icon!("twitch"),
        "speaker" => fa_icon!("speaker"),
        "mixer" => fa_icon!("mixer"),
        _ => fa_icon!("mic"),
    }
}

pub fn get_deactivated(name: &str) -> IconData {
    match name {
        "mic" => fa_icon!("mic_deactivated"),
        "headset" => fa_icon!("headset_self_deactivated"),
        "headphones" => fa_icon!("headphones_self_deactivated"),
        "stereo" => fa_icon!("stereo_self_deactivated"),
        "jack" => fa_icon!("jack_self_deactivated"),
        "dac" => fa_icon!("dac_self_deactivated"),
        "apps" => fa_icon!("apps_self_deactivated"),
        "discord" => fa_icon!("discord_self_deactivated"),
        "obs" => fa_icon!("obs_deactivated"),
        "twitch" => fa_icon!("twitch_self_deactivated"),
        "speaker" => fa_icon!("speaker_deactivated"),
        "mixer" => fa_icon!("mixer_self_deactivated"),
        _ => fa_icon!("mic_deactivated"),
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
