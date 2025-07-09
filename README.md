# input_output

[![Crates.io](https://img.shields.io/crates/v/input_output.svg)](https://crates.io/crates/input_output)
[![Documentation](https://docs.rs/input_output/badge.svg)](https://docs.rs/input_output)
[![License](https://img.shields.io/crates/l/input_output.svg)](LICENSE)

一个用于处理代币交易对计算的 Rust 库，支持高精度数值计算和自定义精度设置。

## 功能特点

- 支持代币交易对汇率计算
- 处理高精度数值（支持最高 38 位精度）
- 自定义代币精度设置
- 安全的数值溢出检查
- 完整的错误处理
- 支持人类可读的价格展示

## 安装

将以下依赖添加到你的 `Cargo.toml` 文件中：

```toml
[dependencies]
input_output = "0.1.2"
```

## 使用示例

### 基本用法

```rust
use input_output::*;

// 创建价格对实例
let price = PairRate::new(
    ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
    (10, 19),  // 汇率 1.9（19/10）
    (24, 24),  // 两个代币都使用24位精度
).unwrap();

// 计算输出金额（输入 1 个 TOKEN_A）
let input_amount = 1_000_000_000_000_000_000_000_000u128; // 1 token with 24 decimals
let output = PairRate::calculate_output_amount(&price, input_amount).unwrap();
println!("Output amount: {}", output);

// 获取人类可读的价格
let human_readable = price.get_human_readable_rate(); // 1.9
```

### 不同精度代币

```rust
// 创建不同精度的价格对
let price = PairRate::new(
    ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
    (1, 1),    // 1:1 汇率
    (18, 6),   // TOKEN_A 使用18位精度，TOKEN_B 使用6位精度
).unwrap();

// 输入 1 个 TOKEN_A（18位精度）
let input_amount = 1_000_000_000_000_000_000u128;
let output = PairRate::calculate_output_amount(&price, input_amount).unwrap();
```

## 常量限制

- `MAX_DECIMALS`: 38 - 支持的最大精度
- `MAX_DECIMAL_DIFF`: 32 - 支持的最大精度差
- `MAX_RATE`: u128::MAX / 2 - 最大安全汇率

## API 文档

详细的 API 文档可以在 [docs.rs](https://docs.rs/input_output) 查看。

### 主要类型

#### PairRate

代币交易对的价格信息结构体：

```rust
pub struct PairRate {
    pub token_pair: (String, String),  // (输入代币, 输出代币)
    pub rate: (u128, u128),            // (输入比率, 输出比率)
    pub decimals: (u8, u8),            // (输入精度, 输出精度)
}
```

### 主要方法

- `PairRate::new()` - 创建新的价格对实例
- `calculate_output_amount()` - 计算输出金额
- `calculate_input_amount()` - 计算所需输入金额
- `get_price_rate()` - 获取精确价格率
- `get_human_readable_rate()` - 获取人类可读的价格率

## 错误处理

库中的所有计算函数都返回 `Result` 类型，可能的错误包括：
- 数值溢出
- 除零错误
- 精度超出范围
- 无效汇率
- 精度差过大

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
- 添加 `PairRate::new()` 构造函数
- 添加安全限制常量
- 改进错误处理
- 添加人类可读的价格率方法
- 完善文档和示例

### 0.1.0 (2024-XX-XX)
- 初始发布
- 实现基本功能
