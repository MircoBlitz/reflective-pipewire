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

/// Render optional title text at the top of the button.
fn title_svg(title: &str, color: &str) -> String {
    if title.is_empty() {
        String::new()
    } else {
        format!(
            r#"<text x="72" y="18" text-anchor="middle" font-family="sans-serif" font-size="14" font-weight="bold" fill="{color}">{title}</text>"#,
            color = color, title = title,
        )
    }
}

/// Render a mute toggle button as SVG.
pub fn mute_button(bg_color: &str, icon_color: &str, icon: &str, muted: bool, title: &str) -> String {
    let svg = icons::get(icon, !muted);
    let colored = apply_colors(svg, bg_color, icon_color);
    let base = svg_strip_close(&colored);
    let title_el = title_svg(title, icon_color);

    format!("{base}\n  {title_el}\n</svg>", base = base, title_el = title_el)
}

/// Render a volume bar button as SVG.
pub fn volume_bar(
    bg_color: &str,
    icon_color: &str,
    bar_color: &str,
    icon: &str,
    volume: f32,
    muted: bool,
    title: &str,
) -> String {
    let svg = icons::get(icon, !muted);
    let colored = apply_colors(svg, bg_color, icon_color);
    let base = svg_strip_close(&colored);

    let bar_width = (volume.clamp(0.0, 1.0) * 120.0) as u32;
    let bar_opacity = if muted { "0.3" } else { "1.0" };
    let icon_opacity = if muted { "0.5" } else { "1.0" };
    let pct = (volume * 100.0).round() as u32;
    let title_el = title_svg(title, icon_color);

    format!(
        r#"{base}
  {title_el}
  <text x="72" y="112" text-anchor="middle" font-family="sans-serif" font-size="18" font-weight="bold" fill="{ic}" opacity="{icon_opacity}">{pct}%</text>
  <rect x="12" y="120" width="120" height="12" rx="6" fill="{ic}" opacity="0.12"/>
  <rect x="12" y="120" width="{bar_w}" height="12" rx="6" fill="{bar_c}" opacity="{bar_opacity}"/>
</svg>"#,
        base = base,
        title_el = title_el,
        ic = icon_color,
        icon_opacity = icon_opacity,
        pct = pct,
        bar_w = bar_width,
        bar_c = bar_color,
        bar_opacity = bar_opacity,
    )
}

/// Render a volume button (up/down/set) as SVG.
pub fn volume_button(bg_color: &str, icon_color: &str, icon: &str, label: &str, title: &str) -> String {
    let svg = icons::get(icon, true);
    let colored = apply_colors(svg, bg_color, icon_color);
    let base = svg_strip_close(&colored);
    let title_el = title_svg(title, icon_color);

    format!(
        r#"{base}
  {title_el}
  <text x="72" y="126" text-anchor="middle" font-family="sans-serif" font-size="20" font-weight="bold" fill="{ic}">{label}</text>
</svg>"#,
        base = base, title_el = title_el, ic = icon_color, label = label,
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