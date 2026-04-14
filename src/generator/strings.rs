//! 生成字符串资源代码的模块。

use log::{debug, info};
use std::collections::HashMap;

/// 生成字符串资源代码
///
/// 产生一个 StringId 枚举和一个基于 match 的高效查找函数。
///
/// # 参数
/// - `keys`: 字符串资源键名列表
/// - `locales`: 支持的语言环境列表
/// - `data`: 字符串资源数据
///
/// # 返回
/// 生成的 Rust 代码字符串
pub fn gen_strings(
    keys: &[String],
    locales: &[String],
    data: &HashMap<String, HashMap<String, String>>,
) -> String {
    debug!("Generating string code for {} keys in {} locales", keys.len(), locales.len());
    let mut code = String::new();

    code.push_str("/// 自动生成的字符串资源 ID\n");
    code.push_str("#[allow(non_camel_case_types)]\n");
    code.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n");
    code.push_str("pub enum StringId {\n");
    for key in keys {
        code.push_str(&format!("    {},\n", key));
    }
    code.push_str("}\n\n");

    code.push_str("/// 根据语言环境返回静态字符串引用 (零拷贝)\n");
    code.push_str("pub fn get_raw_string(id: StringId, locale: &str) -> &'static str {\n");
    code.push_str("    match locale {\n");

    for locale in locales {
        code.push_str(&format!("        {:?} => match id {{\n", locale));
        for key in keys {
            let map = data.get(key).unwrap();
            let val = map
                .get(locale)
                .or_else(|| map.get("default"))
                .map(|s| s.as_str())
                .unwrap_or("");
            code.push_str(&format!("            StringId::{} => {:?},\n", key, val));
        }
        code.push_str("        },\n");
    }

    code.push_str("        _ => match id {\n");
    for key in keys {
        let val = data
            .get(key)
            .unwrap()
            .get("default")
            .map(|s| s.as_str())
            .unwrap_or("");
        code.push_str(&format!("            StringId::{} => {:?},\n", key, val));
    }
    code.push_str("        }\n    }\n}\n");

    info!("Successfully generated string code");
    code
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_gen_strings_simple() {
        let keys = vec!["test_key".to_string()];
        let locales = vec![];
        let mut data = HashMap::new();
        let mut default_map = HashMap::new();
        default_map.insert("default".to_string(), "Test Value".to_string());
        data.insert("test_key".to_string(), default_map);

        let code = gen_strings(&keys, &locales, &data);
        assert!(code.contains("pub enum StringId"));
        assert!(code.contains("test_key"));
        assert!(code.contains("Test Value"));
    }

    #[test]
    fn test_gen_strings_with_multiple_keys() {
        let keys = vec!["key1".to_string(), "key2".to_string()];
        let locales = vec![];
        let mut data = HashMap::new();
        
        let mut map1 = HashMap::new();
        map1.insert("default".to_string(), "Value 1".to_string());
        data.insert("key1".to_string(), map1);

        let mut map2 = HashMap::new();
        map2.insert("default".to_string(), "Value 2".to_string());
        data.insert("key2".to_string(), map2);

        let code = gen_strings(&keys, &locales, &data);
        assert!(code.contains("key1"));
        assert!(code.contains("key2"));
        assert!(code.contains("Value 1"));
        assert!(code.contains("Value 2"));
    }
}
