//! 生成颜色资源代码的模块。

use log::{debug, info};
use std::collections::HashMap;

/// 生成颜色资源代码
///
/// 产生一个 raw 模块存储常量，以及一个带 Theme 感知的查找函数。
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

    code.push_str("#[allow(non_upper_case_globals, dead_code)]\n");
    code.push_str("pub mod raw {\n    use super::Color;\n");
    for key in all_keys {
        let l_val = light_map
            .get(key)
            .cloned()
            .unwrap_or_else(|| "Color::TRANSPARENT".to_string());
        let d_val = dark_map.get(key).cloned().unwrap_or_else(|| l_val.clone());
        code.push_str(&format!(
            "    pub const {}_light: Color = {};\n",
            key, l_val
        ));
        code.push_str(&format!("    pub const {}_dark: Color = {};\n", key, d_val));
    }
    code.push_str("}\n\n");

    for key in all_keys {
        let fn_name = key.to_lowercase();
        code.push_str("#[inline]\n");
        code.push_str(&format!("pub fn {}(theme: &Theme) -> Color {{\n", fn_name));
        code.push_str("    match theme {\n");
        code.push_str(&format!(
            "        Theme::Dark | Theme::Custom(_) => raw::{}_dark,\n",
            key
        ));
        code.push_str(&format!("        _ => raw::{}_light,\n", key));
        code.push_str("    }\n}\n\n");
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
        assert!(code.contains("primary_light"));
        assert!(code.contains("primary_dark"));
        assert!(code.contains("secondary_light"));
        assert!(code.contains("secondary_dark"));
        assert!(code.contains("pub fn primary"));
        assert!(code.contains("pub fn secondary"));
    }
}
