//! 生成尺寸资源代码的模块。

use log::{debug, info};

/// 生成尺寸资源代码
///
/// # 参数
/// - `items`: (尺寸名, 数值) 的向量
///
/// # 返回
/// 生成的 Rust 代码字符串
pub fn gen_dimens(items: &[(String, f32)]) -> String {
    debug!("Generating dimen code for {} items", items.len());
    let mut code = String::new();
    code.push_str("#[allow(non_upper_case_globals, dead_code)]\n");
    for (name, val) in items {
        code.push_str(&format!(
            "pub const {}: f32 = {:.2};\n",
            name.to_lowercase(),
            val
        ));
    }
    info!("Successfully generated dimen code");
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_dimens() {
        let items = vec![("margin".to_string(), 16.0), ("padding".to_string(), 8.0)];
        let code = gen_dimens(&items);
        assert!(code.contains("pub const margin"));
        assert!(code.contains("16.00"));
        assert!(code.contains("pub const padding"));
        assert!(code.contains("8.00"));
    }

    #[test]
    fn test_gen_dimens_empty() {
        let items: Vec<(String, f32)> = vec![];
        let code = gen_dimens(&items);
        assert!(code.contains("#[allow(non_upper_case_globals, dead_code)]"));
    }
}
