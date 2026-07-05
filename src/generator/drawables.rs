//! 生成可绘制资源代码的模块。

use log::{debug, info};

/// 生成可绘制资源代码 (重点：OnceLock 缓存)
///
/// # 参数
/// - `items`: (方法名, 相对于 manifest_dir 的路径, 扩展名) 的向量
///
/// # 返回
/// 生成的 Rust 代码字符串
pub fn gen_drawables(items: &[(String, String, String)]) -> String {
    debug!("Generating drawable code for {} items", items.len());
    let mut code = String::new();

    // 只在有资源时才添加 imports
    let has_image = items.iter().any(|(_, _, ext)| ext != "svg");
    let has_svg = items.iter().any(|(_, _, ext)| ext == "svg");
    if has_image {
        code.push_str("use iced::widget::image;\n");
    }
    if has_svg {
        code.push_str("use iced::widget::svg;\n");
    }
    if !items.is_empty() {
        code.push('\n');
    }

    for (name, rel_path, ext) in items {
        let (handle_type, load_method) = match ext.as_str() {
            "svg" => ("svg::Handle", "from_memory"),
            _ => ("image::Handle", "from_bytes"),
        };
        code.push_str(&format!(
            r#"#[inline]
pub fn {name}() -> {handle_type} {{
    static HANDLE: std::sync::OnceLock<{handle_type}> = std::sync::OnceLock::new();
    HANDLE.get_or_init(|| {{
        static BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/{rel_path}"));
        {handle_type}::{load_method}(BYTES)
    }}).clone()
}}

"#,
            name = name,
            handle_type = handle_type,
            rel_path = rel_path,
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
        let items = vec![("logo".to_string(), "res/drawable/logo.png".to_string(), "png".to_string())];
        let code = gen_drawables(&items);
        assert!(code.contains("pub fn logo()"));
        assert!(code.contains("image::Handle"));
        assert!(code.contains("from_bytes"));
        assert!(code.contains("CARGO_MANIFEST_DIR"));
        assert!(code.contains("res/drawable/logo.png"));
        // 有 png 时应包含 image import，不包含 svg import
        assert!(code.contains("use iced::widget::image;"));
        assert!(!code.contains("use iced::widget::svg;"));
    }

    #[test]
    fn test_gen_drawables_svg() {
        let items = vec![("icon".to_string(), "res/drawable/icon.svg".to_string(), "svg".to_string())];
        let code = gen_drawables(&items);
        assert!(code.contains("pub fn icon()"));
        assert!(code.contains("svg::Handle"));
        assert!(code.contains("from_memory"));
        assert!(code.contains("CARGO_MANIFEST_DIR"));
        // 有 svg 时应包含 svg import，不包含 image import
        assert!(code.contains("use iced::widget::svg;"));
        assert!(!code.contains("use iced::widget::image;"));
    }

    #[test]
    fn test_gen_drawables_empty() {
        let items: Vec<(String, String, String)> = vec![];
        let code = gen_drawables(&items);
        // 无资源时不应包含任何 import
        assert!(!code.contains("use iced::widget::image;"));
        assert!(!code.contains("use iced::widget::svg;"));
    }
}
