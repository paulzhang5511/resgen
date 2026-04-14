//! 生成总入口文件代码的模块。

use log::{debug, info};

/// 生成总入口文件代码 (r.rs 的内容)
///
/// # 返回
/// 生成的总入口文件代码字符串
pub fn gen_r_entry() -> String {
    debug!("Generating R entry code");
    let code = r#"
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[doc(hidden)]
pub mod color {
    include!(concat!(env!("OUT_DIR"), "/colors_generated.rs"));
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[doc(hidden)]
pub mod dimen {
    include!(concat!(env!("OUT_DIR"), "/dimens_generated.rs"));
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[doc(hidden)]
pub mod drawable {
    include!(concat!(env!("OUT_DIR"), "/drawable_generated.rs"));
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[doc(hidden)]
pub mod strings {
    use once_cell::sync::Lazy;
    use std::sync::RwLock;

    include!(concat!(env!("OUT_DIR"), "/strings_generated.rs"));

    static CURRENT_LOCALE: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new("zh-rCN".to_string()));

    pub fn set_locale(lang_code: &str) {
        let mut lock = CURRENT_LOCALE.write().unwrap();
        if *lock != lang_code {
            *lock = lang_code.to_string();
        }
    }

    pub fn current_locale() -> String {
        CURRENT_LOCALE.read().unwrap().clone()
    }

    #[inline]
    pub fn get(id: StringId) -> &'static str {
        let lock = CURRENT_LOCALE.read().unwrap();
        get_raw_string(id, &lock)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[doc(hidden)]
pub mod R {
    pub use super::strings::StringId as string;
    pub use super::color;
    pub use super::dimen;
    pub use super::drawable;
    use super::strings;
    use super::R;

    #[inline]
    pub fn get_string(id: R::string) -> String {
        strings::get(id).to_string()
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
    }
}
