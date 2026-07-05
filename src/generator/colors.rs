//! 生成颜色资源代码的模块。

use log::{debug, info};
use std::collections::HashMap;

/// 生成颜色资源代码
///
/// 产生私有的 UPPER_SNAKE_CASE 常量和主题感知的公开函数。
/// 当浅色/深色值相同时，只生成一个常量并直接返回。
///
/// # 参数
/// - `all_keys`: 所有颜色资源键名列表
/// - `light_map`: 浅色模式颜色映射
/// - `dark_map`: 深色模式颜色映射
///
/// # 返回
/// 生成的 Rust 代码字符串
pub fn gen_colors(
    all_keys: &[String],
    light_map: &HashMap<String, String>,
    dark_map: &HashMap<String, String>,
) -> String {
    debug!("Generating color code for {} keys", all_keys.len());
    let mut code = String::from("use iced::Color;\nuse iced::Theme;\n\n");

    // 生成私有常量
    for key in all_keys {
        let const_name = key.to_uppercase();
        let l_val = light_map
            .get(key)
            .cloned()
            .unwrap_or_else(|| "Color::TRANSPARENT".to_string());
        let d_val = dark_map.get(key).cloned().unwrap_or_else(|| l_val.clone());

        if l_val == d_val {
            // 浅色/深色相同，只生成一个常量
            code.push_str(&format!("const {}: Color = {};\n", const_name, l_val));
        } else {
            code.push_str(&format!("const {}_LIGHT: Color = {};\n", const_name, l_val));
            code.push_str(&format!("const {}_DARK: Color = {};\n", const_name, d_val));
        }
    }
    code.push('\n');

    // 生成公开函数
    for key in all_keys {
        let fn_name = key.to_lowercase();
        let const_name = key.to_uppercase();
        let l_val = light_map
            .get(key)
            .cloned()
            .unwrap_or_else(|| "Color::TRANSPARENT".to_string());
        let d_val = dark_map.get(key).cloned().unwrap_or_else(|| l_val.clone());

        code.push_str("#[inline]\n");
        if l_val == d_val {
            code.push_str(&format!("pub fn {}(_: &Theme) -> Color {{\n", fn_name));
            code.push_str(&format!("    {}\n", const_name));
            code.push_str("}\n\n");
        } else {
            code.push_str(&format!("pub fn {}(theme: &Theme) -> Color {{\n", fn_name));
            code.push_str("    match theme {\n");
            code.push_str(&format!("        Theme::Dark => {}_DARK,\n", const_name));
            code.push_str(&format!("        _ => {}_LIGHT,\n", const_name));
            code.push_str("    }\n}\n\n");
        }
    }

    info!("Successfully generated color code");
    code
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_gen_colors() {
        let keys = vec!["primary".to_string(), "secondary".to_string()];
        let mut light_map = HashMap::new();
        light_map.insert("primary".to_string(), "Color { r: 0.204, g: 0.596, b: 0.859, a: 1.000 }".to_string());
        light_map.insert("secondary".to_string(), "Color { r: 0.180, g: 0.800, b: 0.443, a: 1.000 }".to_string());

        let mut dark_map = HashMap::new();
        dark_map.insert("primary".to_string(), "Color { r: 0.365, g: 0.867, b: 0.886, a: 1.000 }".to_string());
        dark_map.insert("secondary".to_string(), "Color { r: 0.345, g: 0.839, b: 0.553, a: 1.000 }".to_string());

        let code = gen_colors(&keys, &light_map, &dark_map);
        assert!(code.contains("PRIMARY_LIGHT"));
        assert!(code.contains("PRIMARY_DARK"));
        assert!(code.contains("SECONDARY_LIGHT"));
        assert!(code.contains("SECONDARY_DARK"));
        assert!(code.contains("pub fn primary"));
        assert!(code.contains("pub fn secondary"));
        // 不应有 raw 模块
        assert!(!code.contains("pub mod raw"));
    }

    #[test]
    fn test_gen_colors_same_light_dark() {
        let keys = vec!["accent".to_string()];
        let mut light_map = HashMap::new();
        light_map.insert("accent".to_string(), "Color { r: 1.000, g: 0.000, b: 0.000, a: 1.000 }".to_string());

        let mut dark_map = HashMap::new();
        dark_map.insert("accent".to_string(), "Color { r: 1.000, g: 0.000, b: 0.000, a: 1.000 }".to_string());

        let code = gen_colors(&keys, &light_map, &dark_map);
        // 相同时应只生成一个常量，不区分 LIGHT/DARK
        assert!(code.contains("const ACCENT: Color"));
        assert!(!code.contains("ACCENT_LIGHT"));
        assert!(!code.contains("ACCENT_DARK"));
        // 函数应忽略 theme 参数
        assert!(code.contains("pub fn accent(_: &Theme)"));
    }
}
