//! 扫描 drawable 目录的模块。

use log::{debug, info};
use std::path::Path;
use walkdir::WalkDir;

/// 扫描 Drawables (提取路径和扩展名)
///
/// # 参数
/// - `res_dir`: 资源目录的路径
/// - `manifest_dir`: Cargo manifest 目录路径，用于计算可移植的相对路径
///
/// # 返回
/// 成功返回 (方法名, 相对路径, 扩展名) 的向量，失败返回错误。
/// 目录不存在时返回空向量（不算错误）。
#[allow(clippy::type_complexity)]
pub fn parse_drawables(res_dir: &Path, manifest_dir: &Path) -> Result<Vec<(String, String, String)>, Box<dyn std::error::Error>> {
    let mut items = Vec::new();
    let dir = res_dir.join("drawable");

    debug!("Starting to parse drawables from: {}", dir.display());

    if !dir.exists() {
        info!("drawable directory not found at: {}", dir.display());
        return Ok(items);
    }

    for entry in WalkDir::new(&dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let stem = path.file_stem().unwrap().to_str().unwrap().to_string();
            let ext = path
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap()
                .to_string();
            // 使用相对于 manifest_dir 的路径，确保 include_bytes! 的可移植性
            let rel_path = path
                .strip_prefix(manifest_dir)
                .unwrap_or(path)
                .to_str()
                .unwrap()
                .replace('\\', "/");
            debug!("Found drawable: {} with ext {}", stem, ext);
            items.push((stem, rel_path, ext));
        }
    }

    info!("Successfully parsed {} drawables", items.len());
    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_parse_drawables_nonexistent() {
        let temp_dir = tempdir().unwrap();
        let drawables = parse_drawables(temp_dir.path(), temp_dir.path()).unwrap();
        assert!(drawables.is_empty());
    }

    #[test]
    fn test_parse_drawables_with_files() {
        let temp_dir = tempdir().unwrap();
        let res_dir = temp_dir.path().join("res");
        let drawable_dir = res_dir.join("drawable");
        fs::create_dir_all(&drawable_dir).unwrap();

        let png_file = drawable_dir.join("logo.png");
        fs::write(&png_file, "test png data").unwrap();

        let svg_file = drawable_dir.join("icon.svg");
        fs::write(&svg_file, "<svg></svg>").unwrap();

        let drawables = parse_drawables(&res_dir, temp_dir.path()).unwrap();
        assert_eq!(drawables.len(), 2);

        let names: Vec<&str> = drawables.iter().map(|(n, _, _)| n.as_str()).collect();
        assert!(names.contains(&"logo"));
        assert!(names.contains(&"icon"));

        let exts: Vec<&str> = drawables.iter().map(|(_, _, e)| e.as_str()).collect();
        assert!(exts.contains(&"png"));
        assert!(exts.contains(&"svg"));

        // 验证路径是相对路径
        for (_, path, _) in &drawables {
            assert!(!path.starts_with('/') && !path.starts_with('\\'),
                "path should be relative, got: {}", path);
        }
    }
}
