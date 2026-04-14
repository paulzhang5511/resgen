//! 解析 colors.xml 的模块。

use log::{debug, info, warn};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::parser::utils::parse_hex_color;

/// 解析 Colors.xml (处理十六进制颜色)
/// 
/// # 参数
/// - `res_dir`: 资源目录的路径
/// - `is_night`: 是否解析深色模式颜色
/// 
/// # 返回
/// 颜色名 -> 颜色值的映射
pub fn parse_colors(res_dir: &Path, is_night: bool) -> HashMap<String, String> {
    let sub_dir = if is_night { "values-night" } else { "values" };
    let path = res_dir.join(sub_dir).join("colors.xml");
    let mut map = HashMap::new();

    debug!("Starting to parse colors from: {}", path.display());

    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(doc) = roxmltree::Document::parse(&content) {
            for node in doc.descendants().filter(|n| n.has_tag_name("color")) {
                if let Some(name) = node.attribute("name") {
                    let hex = node.text().unwrap_or("#000000").trim();
                    if let Some((r, g, b, a)) = parse_hex_color(hex) {
                        let color_str = format!(
                            "Color {{ r: {:.3}, g: {:.3}, b: {:.3}, a: {:.3} }}",
                            r, g, b, a
                        );
                        debug!("Parsed color '{}': {}", name, color_str);
                        map.insert(name.to_string(), color_str);
                    } else {
                        warn!("Failed to parse color '{}' with value '{}'", name, hex);
                    }
                }
            }
        } else {
            warn!("Failed to parse colors.xml at: {}", path.display());
        }
    } else {
        info!("colors.xml not found at: {}", path.display());
    }

    info!("Successfully parsed {} colors", map.len());
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_parse_colors() {
        let temp_dir = tempdir().unwrap();
        let values_dir = temp_dir.path().join("values");
        fs::create_dir(&values_dir).unwrap();
        let colors_xml = values_dir.join("colors.xml");
        fs::write(&colors_xml, r#"<?xml version="1.0" encoding="utf-8"?>
<resources>
    <color name="primary">#3498db</color>
    <color name="secondary">#2ecc71</color>
    <color name="background">#ffffff</color>
    <color name="text">#333333</color>
</resources>"#).unwrap();

        let colors = parse_colors(temp_dir.path(), false);
        assert_eq!(colors.len(), 4);
        assert!(colors.contains_key("primary"));
        assert!(colors.contains_key("secondary"));
        assert!(colors.contains_key("background"));
        assert!(colors.contains_key("text"));
    }

    #[test]
    fn test_parse_colors_nonexistent_file() {
        let temp_dir = tempdir().unwrap();
        let colors = parse_colors(temp_dir.path(), false);
        assert!(colors.is_empty());
    }
}
