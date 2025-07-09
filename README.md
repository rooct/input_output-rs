# input_output

[![Crates.io](https://img.shields.io/crates/v/input_output.svg)](https://crates.io/crates/input_output)
[![Documentation](https://docs.rs/input_output/badge.svg)](https://docs.rs/input_output)
[![License](https://img.shields.io/crates/l/input_output.svg)](LICENSE)

一个用于处理大数输入输出的 Rust 库。

## 功能特点

- 支持大数运算
- 高效的输入输出处理
- 简单易用的 API

## 安装

将以下依赖添加到你的 `Cargo.toml` 文件中：

```toml
[dependencies]
input_output = "0.1.0"
```

## 使用示例

```rust
use input_output::*;

 let price = PairRate {
    token_pair: ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
    rate: (10, 19), // 1 input_token = 2 output_token
    decimals: (24, 24),
};

// 输入1个token（24位精度）
let input_amount = 200_000_000_000_000_000_000_000_000u128; // 1 token with 24 decimals
let output = PairRate::calculate_output_amount(&price, input_amount).unwrap();
println!("Output amount: {}", output);
```

## API 文档

详细的 API 文档可以在 [docs.rs](https://docs.rs/input_output) 查看。

## 最低支持的 Rust 版本（MSRV）

此库需要 Rust 1.56.0 或更高版本。

## License

此项目采用 MIT/Apache-2.0 双重许可。

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

## 贡献

欢迎提交 Issues 和 Pull Requests！

## 更新日志

### 0.1.0 (2024-XX-XX)
- 初始发布
- 实现基本功能
