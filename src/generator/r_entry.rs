//! 生成总入口文件代码的模块。

use log::{debug, info};

/// 生成总入口文件代码 (r.rs 的内容)
///
/// # 返回
/// 生成的总入口文件代码字符串
pub fn gen_r_entry() -> String {
    debug!("Generating R entry code");
    let code = r#"
#[allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]
#[doc(hidden)]
pub mod color {
    include!("colors_generated.rs");
}

#[allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]
#[doc(hidden)]
pub mod dimen {
    include!("dimens_generated.rs");
}

#[allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]
#[doc(hidden)]
pub mod drawable {
    include!("drawable_generated.rs");
}

#[allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]
#[doc(hidden)]
pub mod strings {
    use std::sync::OnceLock;
    use std::sync::RwLock;

    include!("strings_generated.rs");

    static CURRENT_LOCALE: OnceLock<RwLock<String>> = OnceLock::new();

    fn current_locale_lock() -> &'static RwLock<String> {
        CURRENT_LOCALE.get_or_init(|| RwLock::new("default".to_string()))
    }

    pub fn set_locale(lang_code: &str) {
        let lock = current_locale_lock();
        let mut guard = lock.write().unwrap();
        if *guard != lang_code {
            *guard = lang_code.to_string();
        }
    }

    pub fn current_locale() -> String {
        current_locale_lock().read().unwrap().clone()
    }

    #[inline]
    pub fn get(id: StringId) -> &'static str {
        let lock = current_locale_lock();
        let guard = lock.read().unwrap();
        get_raw_string(id, &guard)
    }
}

#[allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]
#[doc(hidden)]
pub mod R {
    pub use super::strings::StringId as string;
    pub use super::color;
    pub use super::dimen;
    pub use super::drawable;
    use super::strings;

    #[inline]
    pub fn get_string(id: string) -> &'static str {
        strings::get(id)
    }
}
"#
    .to_string();

    info!("Successfully generated R entry code");
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_r_entry() {
        let code = gen_r_entry();
        assert!(code.contains("pub mod color"));
        assert!(code.contains("pub mod dimen"));
        assert!(code.contains("pub mod drawable"));
        assert!(code.contains("pub mod strings"));
        assert!(code.contains("pub mod R"));
        assert!(code.contains("get_string"));
        // 使用相对路径 include，而非 OUT_DIR
        assert!(code.contains("include!(\"colors_generated.rs\")"));
        assert!(code.contains("include!(\"dimens_generated.rs\")"));
        assert!(code.contains("include!(\"drawable_generated.rs\")"));
        assert!(code.contains("include!(\"strings_generated.rs\")"));
        // 合并的 #[allow]，每个模块一个
        assert!(code.contains("#[allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]"));
        // 不应有 4 个分散的 #[allow]
        assert!(!code.contains("once_cell"));
        // 使用 OnceLock 而非 once_cell
        assert!(code.contains("OnceLock"));
        // get_string 返回 &str 而非 String
        assert!(code.contains("-> &'static str"));
        // 无自引用
        assert!(!code.contains("use super::R;"));
    }
}
