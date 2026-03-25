//! 简单的测试脚本来验证统计插件功能

use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() {
    println!("测试统计插件功能...");

    // 启动 basic_pet 示例
    let mut child = Command::new("cargo")
        .args(&["run", "--example", "basic_pet", "--features", "wasm-plugin"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start basic_pet example");

    // 等待插件加载
    thread::sleep(Duration::from_secs(3));

    // 杀死进程（测试完成）
    child.kill().expect("Failed to kill process");

    // 读取输出
    let output = child.wait_with_output().expect("Failed to read output");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("=== STDOUT ===");
    println!("{}", stdout);
    println!("=== STDERR ===");
    println!("{}", stderr);

    // 检查插件是否加载成功
    if stdout.contains("Stats WASM plugin loaded successfully") {
        println!("✅ 统计插件加载成功");
    } else {
        println!("❌ 统计插件加载失败");
    }

    if stdout.contains("Total WASM plugins: 2") {
        println!("✅ 两个插件都加载成功");
    } else {
        println!("❌ 插件数量不对");
    }
}
