fn main() {
    resgen::Config::new()
        .res_dir("res")
        .build()
        .expect("资源生成失败");
}
