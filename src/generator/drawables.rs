//! 生成可绘制资源代码的模块。

use log::{debug, info};

/// 生成可绘制资源代码 (重点：OnceLock 缓存)
///
/// # 参数
/// - `items`: (方法名, 绝对路径, 扩展名) 的向量
///
/// # 返回
/// 生成的 Rust 代码字符串
pub fn gen_drawables(items: &[(String, String, String)]) -> String {
    debug!("Generating drawable code for {} items", items.len());
    let mut code = String::from("use iced::widget::image;\nuse iced::widget::svg;\n\n");

    for (name, abs_path, ext) in items {
        let (handle_type, load_method) = match ext.as_str() {
            "svg" => ("svg::Handle", "from_memory"),
            _ => ("image::Handle", "from_bytes"),
        };
        let safe_path = abs_path.replace('\\', "/");
        code.push_str(&format!(
            r#"#[inline]
pub fn {name}() -> {handle_type} {{
    static HANDLE: std::sync::OnceLock<{handle_type}> = std::sync::OnceLock::new();
    HANDLE.get_or_init(|| {{
        static BYTES: &[u8] = include_bytes!(r"{path}");
        {handle_type}::{load_method}(BYTES)
    }}).clone()
}}

"#,
            name = name,
            handle_type = handle_type,
            path = safe_path,
            load_method = load_method
        ));
    }
    info!("Successfully generated drawable code");
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_drawables_png() {
        let items = vec![("logo".to_string(), "/tmp/logo.png".to_string(), "png".to_string())];
        let code = gen_drawables(&items);
        assert!(code.contains("pub fn logo()"));
        assert!(code.contains("image::Handle"));
        assert!(code.contains("from_bytes"));
        assert!(code.contains("/tmp/logo.png"));
    }

    #[test]
    fn test_gen_drawables_svg() {
        let items = vec![("icon".to_string(), "/tmp/icon.svg".to_string(), "svg".to_string())];
        let code = gen_drawables(&items);
        assert!(code.contains("pub fn icon()"));
        assert!(code.contains("svg::Handle"));
        assert!(code.contains("from_memory"));
    }
}
