//! 辅助工具函数，用于解析颜色等。



/// 辅助函数：解析 Android 十六进制颜色 (#RGB, #ARGB, #RRGGBB, #AARRGGBB)
/// 
/// # 参数
/// - `hex`: 十六进制颜色字符串
/// 
/// # 返回
/// 成功返回 (r, g, b, a) 元组（范围 0.0-1.0），失败返回 None
pub fn parse_hex_color(hex: &str) -> Option<(f32, f32, f32, f32)> {
    let hex = hex.trim_start_matches('#');
    let (a, r, g, b) = match hex.len() {
        6 => (
            255,
            u8::from_str_radix(&hex[0..2], 16).ok()?,
            u8::from_str_radix(&hex[2..4], 16).ok()?,
            u8::from_str_radix(&hex[4..6], 16).ok()?,
        ),
        8 => (
            u8::from_str_radix(&hex[0..2], 16).ok()?,
            u8::from_str_radix(&hex[2..4], 16).ok()?,
            u8::from_str_radix(&hex[4..6], 16).ok()?,
            u8::from_str_radix(&hex[6..8], 16).ok()?,
        ),
        3 => {
            let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
            (
                255,
                r * 16 + r,
                g * 16 + g,
                b * 16 + b,
            )
        },
        4 => {
            let a = u8::from_str_radix(&hex[0..1], 16).ok()?;
            let r = u8::from_str_radix(&hex[1..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..3], 16).ok()?;
            let b = u8::from_str_radix(&hex[3..4], 16).ok()?;
            (
                a * 16 + a,
                r * 16 + r,
                g * 16 + g,
                b * 16 + b,
            )
        },
        _ => return None,
    };
    Some((
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_color_valid_formats() {
        assert_eq!(parse_hex_color("#000000"), Some((0.0, 0.0, 0.0, 1.0)));
        assert_eq!(parse_hex_color("#FFFFFFFF"), Some((1.0, 1.0, 1.0, 1.0)));
        assert_eq!(parse_hex_color("#FFF"), Some((1.0, 1.0, 1.0, 1.0)));
        assert_eq!(parse_hex_color("#80FF0000"), Some((1.0, 0.0, 0.0, 0.5019608)));
        assert_eq!(parse_hex_color("#0F0F"), Some((1.0, 0.0, 1.0, 0.0)));
    }

    #[test]
    fn test_parse_hex_color_invalid() {
        assert_eq!(parse_hex_color("#RRGGBB"), None);
        assert_eq!(parse_hex_color(""), None);
        assert_eq!(parse_hex_color("#"), None);
        assert_eq!(parse_hex_color("#00000"), None);
        assert_eq!(parse_hex_color("#0000000"), None);
        assert_eq!(parse_hex_color("#GGGGGG"), None);
    }
}
