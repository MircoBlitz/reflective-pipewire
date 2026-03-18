pub mod icons;

/// Parse a hex color string to (r, g, b).
fn parse_hex(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    (r, g, b)
}

/// Interpolate between two hex colors. t=0.0 → from, t=1.0 → to.
pub fn lerp_color(from: &str, to: &str, t: f32) -> String {
    let (r1, g1, b1) = parse_hex(from);
    let (r2, g2, b2) = parse_hex(to);
    let t = t.clamp(0.0, 1.0);
    let r = (r1 as f32 + (r2 as f32 - r1 as f32) * t) as u8;
    let g = (g1 as f32 + (g2 as f32 - g1 as f32) * t) as u8;
    let b = (b1 as f32 + (b2 as f32 - b1 as f32) * t) as u8;
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

/// Apply custom colors to an icon SVG.
/// Replaces the default background (#0a0a0a) and icon fill (#ffffff).
fn apply_colors(svg: &str, bg_color: &str, icon_color: &str) -> String {
    svg.replace("#0a0a0a", bg_color)
       .replace("#ffffff", icon_color)
}

/// Strip the closing </svg> tag so overlay elements can be appended.
fn svg_strip_close(svg: &str) -> &str {
    svg.trim_end().trim_end_matches("</svg>")
}

/// Title rendering options.
pub struct TitleOpts<'a> {
    pub text: &'a str,
    pub color: &'a str,
    pub size: u32,
    pub position: &'a str, // "top", "middle", "bottom"
}

/// Render optional title text.
fn title_svg(opts: &TitleOpts) -> String {
    if opts.text.is_empty() {
        return String::new();
    }
    let y = match opts.position {
        "middle" => 72 + opts.size as i32 / 3,
        "bottom" => 140,
        _ => 4 + opts.size as i32, // top
    };
    format!(
        r#"<text x="72" y="{y}" text-anchor="middle" font-family="sans-serif" font-size="{sz}" font-weight="bold" fill="{c}">{t}</text>"#,
        y = y, sz = opts.size, c = opts.color, t = opts.text,
    )
}

/// Render a mute toggle button as SVG.
pub fn mute_button(bg_color: &str, icon_color: &str, icon: &str, muted: bool, title: &TitleOpts) -> String {
    let svg = icons::get(icon, !muted);
    let colored = apply_colors(svg, bg_color, icon_color);
    let base = svg_strip_close(&colored);
    let title_el = title_svg(title);

    format!("{base}\n  {title_el}\n</svg>", base = base, title_el = title_el)
}

/// Render a volume knob as SVG. show_percent=true shows %, false shows bar.
pub fn volume_knob(
    bg_color: &str,
    icon_color: &str,
    icon: &str,
    volume: f32,
    muted: bool,
    title: &TitleOpts,
    show_percent: bool,
) -> String {
    let svg = icons::get(icon, !muted);
    let colored = apply_colors(svg, bg_color, icon_color);
    let base = svg_strip_close(&colored);

    let bar_opacity = if muted { "0.3" } else { "1.0" };
    let icon_opacity = if muted { "0.5" } else { "1.0" };
    let pct = (volume * 100.0).round() as u32;
    let title_el = title_svg(title);

    let indicator = if show_percent {
        format!(
            r#"<text x="72" y="126" text-anchor="middle" font-family="sans-serif" font-size="18" font-weight="bold" fill="{ic}" opacity="{io}">{pct}%</text>"#,
            ic = icon_color, io = icon_opacity, pct = pct,
        )
    } else {
        let bar_width = (volume.clamp(0.0, 1.0) * 120.0) as u32;
        format!(
            r#"<rect x="12" y="120" width="120" height="12" rx="6" fill="{ic}" opacity="0.12"/>
  <rect x="12" y="120" width="{bw}" height="12" rx="6" fill="{bc}" opacity="{bo}"/>"#,
            ic = icon_color, bw = bar_width, bc = icon_color, bo = bar_opacity,
        )
    };

    format!(
        r#"{base}
  {title_el}
  {indicator}
</svg>"#,
        base = base, title_el = title_el, indicator = indicator,
    )
}

/// Render a volume button (up/down/set) as SVG.
pub fn volume_button(bg_color: &str, icon_color: &str, icon: &str, label: &str, title: &TitleOpts) -> String {
    let svg = icons::get(icon, true);
    let colored = apply_colors(svg, bg_color, icon_color);
    let base = svg_strip_close(&colored);
    let title_el = title_svg(title);
    let white = "#ffffff";

    format!(
        r#"{base}
  <text x="72" y="72" text-anchor="middle" font-family="sans-serif" font-size="40" font-weight="bold" fill="{w}">{label}</text>
  {title_el}
</svg>"#,
        base = base, w = white, label = label, title_el = title_el,
    )
}

/// Build device name element, centered. Auto-wraps to 2 lines and shrinks font if needed.
fn device_name_svg(name: &str, color: &str) -> String {
    let max_w: f32 = 128.0;
    let base_size: f32 = 22.0;
    let char_w_ratio: f32 = 0.55; // avg char width / font size

    let text_w = name.len() as f32 * base_size * char_w_ratio;

    if text_w <= max_w {
        // Single line, fits at base size
        return format!(
            r#"<text x="72" y="85" text-anchor="middle" font-family="sans-serif" font-size="22" fill="{c}" opacity="0.7">{n}</text>"#,
            c = color, n = name,
        );
    }

    // Try to split into 2 lines at a space near the middle
    let mid = name.len() / 2;
    let split = name[..mid].rfind(' ')
        .or_else(|| name[mid..].find(' ').map(|i| i + mid));

    if let Some(pos) = split {
        let line1 = &name[..pos];
        let line2 = &name[pos + 1..];
        let longest = line1.len().max(line2.len()) as f32;
        // Shrink font to fit the longest line, min 12px
        let size = (max_w / (longest * char_w_ratio)).min(base_size).max(12.0) as u32;
        let y1 = 78;
        let y2 = 78 + size + 3;
        format!(
            r#"<text x="72" y="{y1}" text-anchor="middle" font-family="sans-serif" font-size="{sz}" fill="{c}" opacity="0.7">{l1}</text>
  <text x="72" y="{y2}" text-anchor="middle" font-family="sans-serif" font-size="{sz}" fill="{c}" opacity="0.7">{l2}</text>"#,
            y1 = y1, y2 = y2, sz = size, c = color, l1 = line1, l2 = line2,
        )
    } else {
        // No space to split — shrink to fit single line
        let size = (max_w / (name.len() as f32 * char_w_ratio)).max(12.0) as u32;
        format!(
            r#"<text x="72" y="85" text-anchor="middle" font-family="sans-serif" font-size="{sz}" fill="{c}" opacity="0.7">{n}</text>"#,
            sz = size, c = color, n = name,
        )
    }
}

/// Render a volume display: large percentage + device name, no icon.
pub fn volume_display(
    bg_color: &str,
    text_color: &str,
    volume: f32,
    muted: bool,
    device_name: &str,
    title: &TitleOpts,
) -> String {
    let pct = (volume * 100.0).round() as u32;
    let bar_width = (volume.clamp(0.0, 1.0) * 120.0) as u32;
    let bar_opacity = if muted { "0.3" } else { "1.0" };
    let text_opacity = if muted { "0.4" } else { "1.0" };
    let title_el = title_svg(title);
    let name_el = device_name_svg(device_name, text_color);

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 144 144">
  <rect width="144" height="144" rx="16" fill="{bg}"/>
  {title_el}
  <text x="72" y="45" text-anchor="middle" font-family="sans-serif" font-size="48" font-weight="bold" fill="{tc}" opacity="{to}">{pct}%</text>
  {name_el}
  <rect x="12" y="120" width="120" height="12" rx="6" fill="{tc}" opacity="0.12"/>
  <rect x="12" y="120" width="{bar_w}" height="12" rx="6" fill="{bar_c}" opacity="{bar_o}"/>
</svg>"#,
        bg = bg_color,
        title_el = title_el,
        tc = text_color,
        to = text_opacity,
        pct = pct,
        name_el = name_el,
        bar_w = bar_width,
        bar_c = text_color,
        bar_o = bar_opacity,
    )
}

/// Encode SVG to a data URI suitable for set_image.
pub fn svg_to_data_uri(svg: &str) -> String {
    format!("data:image/svg+xml;base64,{}", base64_encode(svg.as_bytes()))
}

fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = if chunk.len() > 1 { chunk[1] as usize } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as usize } else { 0 };
        out.push(CHARS[b0 >> 2] as char);
        out.push(CHARS[((b0 & 3) << 4) | (b1 >> 4)] as char);
        out.push(if chunk.len() > 1 { CHARS[((b1 & 15) << 2) | (b2 >> 6)] as char } else { '=' });
        out.push(if chunk.len() > 2 { CHARS[b2 & 63] as char } else { '=' });
    }
    out
}