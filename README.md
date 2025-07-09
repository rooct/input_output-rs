# input_output

[![Crates.io](https://img.shields.io/crates/v/input_output.svg)](https://crates.io/crates/input_output)
[![Documentation](https://docs.rs/input_output/badge.svg)](https://docs.rs/input_output)
[![License](https://img.shields.io/crates/l/input_output.svg)](LICENSE)

一个用于处理代币交易对计算的 Rust 库，支持高精度数值计算和自定义精度设置。

## 功能特点

- 支持代币交易对汇率计算
- 处理高精度数值（支持最高 256 位精度）
- 自定义代币精度设置
- 安全的数值溢出检查
- 简单易用的 API

## 安装

将以下依赖添加到你的 `Cargo.toml` 文件中：

```toml
[dependencies]
input_output = "0.1.1"
```

## 使用示例

```rust
use input_output::*;

let price = PairRate {
    token_pair: ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
    rate: (10, 19), // 1 TOKEN_A = 1.9 TOKEN_B
    decimals: (24, 24),
};

// 输入1个token（24位精度）
let input_amount = 200_000_000_000_000_000_000_000_000u128; // 1 token with 24 decimals
let output = PairRate::calculate_output_amount(&price, input_amount).unwrap();
println!("Output amount: {}", output);
```

## 主要类型说明

### PairRate

`PairRate` 结构体用于表示代币交易对的汇率信息：

- `token_pair`: 交易对的代币符号 (input_token, output_token)
- `rate`: 汇率比例 (input_rate, output_rate)
- `decimals`: 代币精度 (input_decimals, output_decimals)

## API 文档

详细的 API 文档可以在 [docs.rs](https://docs.rs/input_output) 查看。

## 错误处理

库中的所有计算函数都返回 `Result` 类型，可能的错误包括：
- 数值溢出
- 除零错误
- 精度超出范围

## 最低支持的 Rust 版本（MSRV）

此库需要 Rust 1.56.0 或更高版本。

## License

此项目采用 MIT/Apache-2.0 双重许可。

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

## 贡献

欢迎提交 Issues 和 Pull Requests！

## 更新日志

### 0.1.1 (2024-XX-XX)
- 改进文档
- 添加详细使用示例
- 完善错误处理

### 0.1.0 (2024-XX-XX)
- 初始发布
- 实现基本功能
