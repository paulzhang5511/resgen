//! 解析 strings.xml 的模块。

use log::{debug, info};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// 将 Android 格式说明符转换为 Rust 格式字符串
///
/// 支持的转换：
/// - `%s` → `{}`（字符串）
/// - `%d` → `{}`（整数）
/// - `%f` → `{}`（浮点数）
/// - `%1$s`, `%2$d` → `{0}`, `{1}`（位置参数，从1开始）
fn convert_android_format(s: &str) -> String {
    let re = Regex::new(r"%(\d+)\$([sdf])|%([sdf])").unwrap();
    let mut result = s.to_string();
    let mut offset = 0i32;

    for cap in re.captures_iter(s) {
        let full = cap.get(0).unwrap();
        let start = (full.start() as i32 + offset) as usize;
        let end = (full.end() as i32 + offset) as usize;

        let replacement = if let Some(index) = cap.get(1) {
            // Positional: %N$X → {N-1}
            format!("{{{}}}", index.as_str().parse::<usize>().unwrap().saturating_sub(1))
        } else {
            // Simple: %X → {}
            "{}".to_string()
        };

        offset += replacement.len() as i32 - (end - start) as i32;
        result.replace_range(start..end, &replacement);
    }

    result
}

/// 字符串资源中间结构
#[derive(Debug, PartialEq)]
pub struct ParsedStrings {
    /// 所有字符串资源键名，已排序
    pub keys: Vec<String>,
    /// 所有支持的语言环境列表，已排序
    pub locales: Vec<String>,
    /// 键名 -> 语言环境 -> 字符串值的映射
    pub data: HashMap<String, HashMap<String, String>>,
}

/// 解析 Strings.xml (支持多语言)
/// 
/// # 参数
/// - `res_dir`: 资源目录的路径
/// 
/// # 返回
/// 解析成功返回 `ParsedStrings`，失败返回错误
pub fn parse_strings(res_dir: &Path) -> Result<ParsedStrings, Box<dyn std::error::Error>> {
    debug!("Starting to parse strings from: {}", res_dir.display());
    let mut string_data: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut all_locales: HashSet<String> = HashSet::new();

    for entry in WalkDir::new(res_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.file_name().and_then(|n| n.to_str()) == Some("strings.xml") {
            debug!("Found strings.xml at: {}", path.display());
            let parent_name = path
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();

            let locale = if parent_name == "values" {
                "default".to_string()
            } else {
                parent_name.trim_start_matches("values-").to_string()
            };

            if locale != "default" {
                debug!("Adding locale: {}", locale);
                all_locales.insert(locale.clone());
            }

            let content = fs::read_to_string(path)?;
            let doc = roxmltree::Document::parse(&content)?;
            for node in doc.descendants().filter(|n| n.has_tag_name("string")) {
                if let Some(name) = node.attribute("name") {
                    let mut val = node.text().unwrap_or("").to_string();
                    val = convert_android_format(&val);
                    val = val.replace("\\'", "'").replace("\\n", "\n");

                    debug!("Parsed string '{}' for locale '{}': {:?}", name, locale, val);
                    string_data
                        .entry(name.to_string())
                        .or_default()
                        .insert(locale.clone(), val);
                }
            }
        }
    }

    let mut keys: Vec<String> = string_data.keys().cloned().collect();
    keys.sort();
    let mut locales: Vec<String> = all_locales.into_iter().collect();
    locales.sort();

    info!("Successfully parsed {} strings in {} locales", keys.len(), locales.len());
    Ok(ParsedStrings {
        keys,
        locales,
        data: string_data,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_parse_strings_simple() {
        let temp_dir = tempdir().unwrap();
        let values_dir = temp_dir.path().join("values");
        fs::create_dir(&values_dir).unwrap();
        let strings_xml = values_dir.join("strings.xml");
        fs::write(&strings_xml, r#"<?xml version="1.0" encoding="utf-8"?>
<resources>
    <string name="app_name">ResGen Test</string>
    <string name="welcome">Hello %s</string>
</resources>"#).unwrap();

        let parsed = parse_strings(temp_dir.path()).unwrap();
        assert_eq!(parsed.keys, vec!["app_name", "welcome"]);
        assert_eq!(parsed.locales.len(), 0);
        assert_eq!(parsed.data.get("app_name").unwrap().get("default").unwrap(), "ResGen Test");
        assert_eq!(parsed.data.get("welcome").unwrap().get("default").unwrap(), "Hello {}");
    }

    #[test]
    fn test_parse_strings_with_special_chars() {
        let temp_dir = tempdir().unwrap();
        let values_dir = temp_dir.path().join("values");
        fs::create_dir(&values_dir).unwrap();
        let strings_xml = values_dir.join("strings.xml");
        fs::write(&strings_xml, r#"<?xml version="1.0" encoding="utf-8"?>
<resources>
    <string name="test1">Hello, world</string>
    <string name="test2">Line 1\nLine 2</string>
    <string name="test3">It\'s a test</string>
</resources>"#).unwrap();

        let parsed = parse_strings(temp_dir.path()).unwrap();
        assert_eq!(parsed.data.get("test1").unwrap().get("default").unwrap(), "Hello, world");
        assert_eq!(parsed.data.get("test2").unwrap().get("default").unwrap(), "Line 1\nLine 2");
        assert_eq!(parsed.data.get("test3").unwrap().get("default").unwrap(), "It's a test");
    }

    #[test]
    fn test_convert_android_format() {
        // Simple format specifiers
        assert_eq!(convert_android_format("Hello %s"), "Hello {}");
        assert_eq!(convert_android_format("Count: %d"), "Count: {}");
        assert_eq!(convert_android_format("Value: %f"), "Value: {}");
        // Positional format specifiers
        assert_eq!(convert_android_format("Hello %1$s, you have %2$d items"), "Hello {0}, you have {1} items");
        // Mixed
        assert_eq!(convert_android_format("%s has %d items"), "{} has {} items");
        // No format specifiers
        assert_eq!(convert_android_format("Plain text"), "Plain text");
    }
}
