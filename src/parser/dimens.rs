//! 解析 dimens.xml 的模块。

use log::{debug, info, warn};
use regex::Regex;
use std::fs;
use std::path::Path;

/// 解析 Dimens.xml
/// 
/// # 参数
/// - `res_dir`: 资源目录的路径
/// 
/// # 返回
/// 尺寸名 -> 数值的向量
pub fn parse_dimens(res_dir: &Path) -> Vec<(String, f32)> {
    let path = res_dir.join("values/dimens.xml");
    let mut dimens = Vec::new();
    let re_num = Regex::new(r"^[0-9\.]+").unwrap();

    debug!("Starting to parse dimens from: {}", path.display());

    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(doc) = roxmltree::Document::parse(&content) {
            for node in doc.descendants().filter(|n| n.has_tag_name("dimen")) {
                if let Some(name) = node.attribute("name") {
                    let text = node.text().unwrap_or("0");
                    if let Some(caps) = re_num.find(text) {
                        if let Ok(val) = caps.as_str().parse::<f32>() {
                            debug!("Parsed dimen '{}': {}", name, val);
                            dimens.push((name.to_string(), val));
                        }
                    }
                }
            }
        } else {
            warn!("Failed to parse dimens.xml at: {}", path.display());
        }
    } else {
        info!("dimens.xml not found at: {}", path.display());
    }

    info!("Successfully parsed {} dimens", dimens.len());
    dimens
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_parse_dimens() {
        let temp_dir = tempdir().unwrap();
        let values_dir = temp_dir.path().join("values");
        fs::create_dir(&values_dir).unwrap();
        let dimens_xml = values_dir.join("dimens.xml");
        fs::write(&dimens_xml, r#"<?xml version="1.0" encoding="utf-8"?>
<resources>
    <dimen name="margin">16.0</dimen>
    <dimen name="padding">8.0</dimen>
    <dimen name="text_size">14.0</dimen>
</resources>"#).unwrap();

        let dimens = parse_dimens(temp_dir.path());
        assert_eq!(dimens.len(), 3);
        let names: Vec<&str> = dimens.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"margin"));
        assert!(names.contains(&"padding"));
        assert!(names.contains(&"text_size"));
        let values: Vec<f32> = dimens.iter().map(|(_, v)| *v).collect();
        assert!(values.contains(&16.0));
        assert!(values.contains(&8.0));
        assert!(values.contains(&14.0));
    }

    #[test]
    fn test_parse_dimens_nonexistent_file() {
        let temp_dir = tempdir().unwrap();
        let dimens = parse_dimens(temp_dir.path());
        assert!(dimens.is_empty());
    }
}
