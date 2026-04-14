//! # 基础示例
//! 
//! 本示例演示了 resgen 的核心功能。

fn main() {
    // 注意：在实际应用中，你需要使用 build.rs 脚本来生成资源，
    // 然后通过以下方式包含它们：
    // include!(concat!(env!("OUT_DIR"), "/r_generated.rs"));

    println!("ResGen 基础示例");
    println!("==============");
    println!();
    println!("本示例展示了 resgen 的工作原理：");
    println!("- 在 Android 风格的目录中组织资源");
    println!("- 在构建时解析资源");
    println!("- 生成类型安全的 Rust 代码");
    println!();
    println!("查看源代码和 README 获取更多详细信息！");
}
