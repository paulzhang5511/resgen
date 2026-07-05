//! 解析 dimens.xml 的模块。

use log::{debug, info};
use regex::Regex;
use std::fs;
use std::path::Path;

/// 解析 Dimens.xml
///
/// # 参数
/// - `res_dir`: 资源目录的路径
///
/// # 返回
/// 成功返回尺寸名 -> 数值的向量，失败返回错误。
/// 文件不存在时返回空向量（不算错误），XML 解析失败则返回错误。
pub fn parse_dimens(res_dir: &Path) -> Result<Vec<(String, f32)>, Box<dyn std::error::Error>> {
    let path = res_dir.join("values/dimens.xml");
    let mut dimens = Vec::new();
    let re_num = Regex::new(r"^[0-9\.]+").unwrap();

    debug!("Starting to parse dimens from: {}", path.display());

    if !path.exists() {
        info!("dimens.xml not found at: {}", path.display());
        return Ok(dimens);
    }

    let content = fs::read_to_string(&path)?;
    let doc = roxmltree::Document::parse(&content).map_err(|e| {
        format!("Failed to parse dimens.xml at {}: {}", path.display(), e)
    })?;

    for node in doc.descendants().filter(|n| n.has_tag_name("dimen")) {
        if let Some(name) = node.attribute("name") {
            let text = node.text().unwrap_or("0");
            if let Some(caps) = re_num.find(text)
                && let Ok(val) = caps.as_str().parse::<f32>()
            {
                debug!("Parsed dimen '{}': {}", name, val);
                dimens.push((name.to_string(), val));
            }
        }
    }

    info!("Successfully parsed {} dimens", dimens.len());
    Ok(dimens)
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

        let dimens = parse_dimens(temp_dir.path()).unwrap();
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
        let dimens = parse_dimens(temp_dir.path()).unwrap();
        assert!(dimens.is_empty());
    }

    #[test]
    fn test_parse_dimens_invalid_xml() {
        let temp_dir = tempdir().unwrap();
        let values_dir = temp_dir.path().join("values");
        fs::create_dir(&values_dir).unwrap();
        let dimens_xml = values_dir.join("dimens.xml");
        fs::write(&dimens_xml, "this is not valid xml <><>").unwrap();

        let result = parse_dimens(temp_dir.path());
        assert!(result.is_err());
    }
}
