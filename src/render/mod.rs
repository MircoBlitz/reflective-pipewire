pub mod icons;

/// Render a mute toggle button as SVG.
/// Icon color reflects state, background is configurable per state.
pub fn mute_button(bg_color: &str, icon_color: &str, icon: &str, muted: bool) -> String {
    let icon_path = icons::get(icon);
    let slash = if muted {
        format!(
            r#"<line x1="28" y1="28" x2="116" y2="116" stroke="{}" stroke-width="8" stroke-linecap="round" opacity="0.9"/>"#,
            icon_color
        )
    } else {
        String::new()
    };

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 144 144">
  <rect width="144" height="144" rx="16" fill="{bg}"/>
  <path fill="{ic}" d="{icon}"/>
  {slash}
</svg>"#,
        bg = bg_color,
        ic = icon_color,
        icon = icon_path,
        slash = slash,
    )
}

/// Render a volume bar button as SVG.
/// Icon color reflects state if reflect_state is true.
pub fn volume_bar(
    bg_color: &str,
    icon_color: &str,
    bar_color: &str,
    icon: &str,
    volume: f32,
    muted: bool,
) -> String {
    let icon_path = icons::get(icon);
    let bar_width = (volume.clamp(0.0, 1.0) * 120.0) as u32;
    let bar_opacity = if muted { "0.3" } else { "1.0" };
    let icon_opacity = if muted { "0.5" } else { "1.0" };
    let pct = (volume * 100.0).round() as u32;
    let track_fill = "#ffffff20";
    let font = "sans-serif";

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 144 144">
  <rect width="144" height="144" rx="16" fill="{bg}"/>
  <g opacity="{icon_opacity}">
    <path fill="{ic}" transform="translate(32,16) scale(0.56)" d="{icon}"/>
  </g>
  <rect x="12" y="112" width="120" height="16" rx="8" fill="{track_fill}"/>
  <rect x="12" y="112" width="{bar_w}" height="16" rx="8" fill="{bar_c}" opacity="{bar_opacity}"/>
  <text x="72" y="105" text-anchor="middle" font-family="{font}" font-size="18" font-weight="bold" fill="{ic}" opacity="{icon_opacity}">{pct}%</text>
</svg>"#,
        bg = bg_color,
        ic = icon_color,
        icon_opacity = icon_opacity,
        icon = icon_path,
        track_fill = track_fill,
        bar_w = bar_width,
        bar_c = bar_color,
        bar_opacity = bar_opacity,
        font = font,
        pct = pct,
    )
}

/// Render a volume button (up/down/set) as SVG.
pub fn volume_button(bg_color: &str, icon_color: &str, icon: &str, label: &str) -> String {
    let icon_path = icons::get(icon);
    let font = "sans-serif";

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 144 144">
  <rect width="144" height="144" rx="16" fill="{bg}"/>
  <g transform="translate(32,10) scale(0.56)">
    <path fill="{ic}" d="{icon}"/>
  </g>
  <text x="72" y="126" text-anchor="middle" font-family="{font}" font-size="20" font-weight="bold" fill="{ic}">{label}</text>
</svg>"#,
        bg = bg_color,
        ic = icon_color,
        icon = icon_path,
        font = font,
        label = label,
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
