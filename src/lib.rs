use num_bigint::BigUint;
use num_traits::ToPrimitive;

/// 常量定义
pub const MAX_DECIMALS: u8 = 38; // 最大支持的精度
pub const MIN_RATE: u128 = 1; // 最小汇率
pub const MAX_RATE: u128 = u128::MAX / 2; // 最大安全汇率
pub const MAX_DECIMAL_DIFF: u8 = 32; // 最大精度差

/// 价格信息结构体
#[derive(Debug, Clone, PartialEq)]
pub struct PairRate {
    pub token_pair: (String, String),
    pub rate: (u128, u128),
    pub decimals: (u8, u8),
}

impl PairRate {
    /// 验证精度是否在有效范围内
    fn validate_decimals(decimals: (u8, u8)) -> Result<(), String> {
        if decimals.0 > MAX_DECIMALS {
            return Err(format!(
                "Input decimals {} exceeds maximum allowed {}",
                decimals.0, MAX_DECIMALS
            ));
        }
        if decimals.1 > MAX_DECIMALS {
            return Err(format!(
                "Output decimals {} exceeds maximum allowed {}",
                decimals.1, MAX_DECIMALS
            ));
        }
        Ok(())
    }

    /// 验证汇率是否在安全范围内
    fn validate_rate(rate: (u128, u128)) -> Result<(), String> {
        if rate.0 < MIN_RATE || rate.1 < MIN_RATE {
            return Err("Rate components must be greater than 0".to_string());
        }
        if rate.0 > MAX_RATE || rate.1 > MAX_RATE {
            return Err(format!("Rate components must be less than {}", MAX_RATE));
        }
        Ok(())
    }

    /// 创建新的价格对实例，带验证
    pub fn new(
        token_pair: (String, String),
        rate: (u128, u128),
        decimals: (u8, u8),
    ) -> Result<Self, String> {
        Self::validate_decimals(decimals)?;
        Self::validate_rate(rate)?;

        Ok(Self {
            token_pair,
            rate,
            decimals,
        })
    }

    /// 根据价格和输入代币数量计算输出代币数量
    pub fn calculate_output_amount(price: &PairRate, input_amount: u128) -> Result<u128, String> {
        if input_amount == 0 {
            return Err("Input amount must be greater than 0".to_string());
        }

        Self::validate_rate(price.rate)?;
        Self::validate_decimals(price.decimals)?;

        let (input_rate, output_rate) = price.rate;

        // 预检查：计算是否可能溢出
        if input_amount > u128::MAX / output_rate {
            return Err("Input amount too large, would cause overflow".to_string());
        }

        // 基础计算：input_amount * output_rate / input_rate
        let base_output = Self::safe_multiply_divide(input_amount, output_rate, input_rate)?;

        // 精度调整：将结果从input_decimals调整到output_decimals
        let adjusted_output =
            Self::adjust_decimals(base_output, price.decimals.0, price.decimals.1)?;

        if adjusted_output == 0 {
            return Err("Calculated output amount is zero, increase input amount".to_string());
        }

        Ok(adjusted_output)
    }

    /// 根据价格和输出代币数量计算需要的输入代币数量
    pub fn calculate_input_amount(price: &PairRate, output_amount: u128) -> Result<u128, String> {
        if output_amount == 0 {
            return Err("Output amount must be greater than 0".to_string());
        }

        Self::validate_rate(price.rate)?;
        Self::validate_decimals(price.decimals)?;

        let (input_rate, output_rate) = price.rate;

        // 预检查：计算是否可能溢出
        if output_amount > u128::MAX / input_rate {
            return Err("Output amount too large, would cause overflow".to_string());
        }

        // 基础计算：output_amount * input_rate / output_rate
        let base_input = Self::safe_multiply_divide(output_amount, input_rate, output_rate)?;

        // 精度调整：将结果从output_decimals调整到input_decimals
        let adjusted_input = Self::adjust_decimals(base_input, price.decimals.1, price.decimals.0)?;

        if adjusted_input == 0 {
            return Err("Calculated input amount is zero, increase output amount".to_string());
        }

        Ok(adjusted_input)
    }

    /// 精度调整函数
    fn adjust_decimals(amount: u128, from_decimals: u8, to_decimals: u8) -> Result<u128, String> {
        // 验证精度范围
        if from_decimals > MAX_DECIMALS || to_decimals > MAX_DECIMALS {
            return Err(format!(
                "Decimals must be less than or equal to {}",
                MAX_DECIMALS
            ));
        }

        // 检查精度差异
        let decimal_diff = if from_decimals > to_decimals {
            from_decimals - to_decimals
        } else {
            to_decimals - from_decimals
        };

        if decimal_diff > MAX_DECIMAL_DIFF {
            return Err(format!(
                "Decimal difference {} exceeds maximum allowed {}",
                decimal_diff, MAX_DECIMAL_DIFF
            ));
        }

        if from_decimals == to_decimals {
            return Ok(amount);
        }

        if from_decimals > to_decimals {
            // 精度降低，需要除法
            let decimal_diff = from_decimals - to_decimals;
            let divisor = 10u128
                .checked_pow(decimal_diff as u32)
                .ok_or("Decimal divisor overflow")?;
            Ok(amount / divisor)
        } else {
            // 精度提高，需要乘法
            let decimal_diff = to_decimals - from_decimals;
            let multiplier = 10u128
                .checked_pow(decimal_diff as u32)
                .ok_or("Decimal multiplier overflow")?;
            amount
                .checked_mul(multiplier)
                .ok_or("Decimal adjustment caused overflow".to_string())
        }
    }

    /// 安全的乘除运算，防止溢出
    fn safe_multiply_divide(amount: u128, multiplier: u128, divisor: u128) -> Result<u128, String> {
        if divisor == 0 {
            return Err("Division by zero".to_string());
        }

        // 预检查：验证输入值范围
        if amount > MAX_RATE || multiplier > MAX_RATE {
            return Err("Input values too large for safe calculation".to_string());
        }

        let amount = BigUint::from(amount);
        let multiplier = BigUint::from(multiplier);
        let divisor = BigUint::from(divisor);

        let result = amount * multiplier / divisor;

        result.to_u128().ok_or("Result exceeds u128".to_string())
    }

    /// 获取价格率，返回比率和精度
    pub fn get_price_rate(&self) -> ((u128, u128), (u8, u8)) {
        (self.rate, self.decimals)
    }

    /// 获取人类可读的价格率（注意：可能损失精度）
    pub fn get_human_readable_rate(&self) -> f64 {
        let (input_rate, output_rate) = self.rate;
        let (input_decimals, output_decimals) = self.decimals;

        let decimal_adjustment = if input_decimals > output_decimals {
            10f64.powi((input_decimals - output_decimals) as i32)
        } else {
            1f64 / 10f64.powi((output_decimals - input_decimals) as i32)
        };

        (output_rate as f64 / input_rate as f64) * decimal_adjustment
    }

    /// 验证价格是否有效
    pub fn is_valid(&self) -> bool {
        Self::validate_rate(self.rate).is_ok() && Self::validate_decimals(self.decimals).is_ok()
    }
}

impl Default for PairRate {
    fn default() -> Self {
        PairRate {
            token_pair: ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            rate: (1, 1),       // 默认1:1价格
            decimals: (18, 18), // 默认18位精度
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_output_amount() {
        let price = PairRate::new(
            ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            (10, 19), // 1 input_token = 1.9 output_token
            (24, 24),
        )
        .unwrap();

        // 输入1个token（24位精度）
        let input_amount = 200_000_000_000_000_000_000_000_000u128; // 1 token with 24 decimals
        let output = PairRate::calculate_output_amount(&price, input_amount).unwrap();
        println!("Output amount: {}", output);
    }

    #[test]
    fn test_calculate_output_amount_r() {
        let price = PairRate::new(
            ("TOKEN_B".to_string(), "TOKEN_A".to_string()),
            (19, 10),
            (24, 24),
        )
        .unwrap();

        // 输入1个token（24位精度）
        let input_amount = 200_000_000_000_000_000_000_000_000u128; // 1 token with 24 decimals
        let output = PairRate::calculate_output_amount(&price, input_amount).unwrap();
        println!("Output amount: {}", output);
    }

    #[test]
    fn test_calculate_input_amount() {
        let price = PairRate::new(
            ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            (1, 2),
            (24, 18),
        )
        .unwrap();

        // 想要输出2个token（18位精度）
        let output_amount = 2_000_000_000_000_000_000u128; // 2 tokens with 18 decimals
        let input = PairRate::calculate_input_amount(&price, output_amount).unwrap();
        println!("Input amount: {}", input);
        // 计算步骤：
        // 1. 基础计算：2_000_000_000_000_000_000 * 1 / 2 = 1_000_000_000_000_000_000
        // 2. 精度调整：从18位调整到24位，乘以10^6 = 1_000_000_000_000_000_000_000_000
        assert_eq!(input, 1_000_000_000_000_000_000_000_000);
    }

    #[test]
    fn test_same_decimals() {
        let price = PairRate::new(
            ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            (1, 3),
            (18, 18),
        )
        .unwrap();

        // 输入1个token（18位精度）
        let input_amount = 1_000_000_000_000_000_000u128; // 1 token
        let output = PairRate::calculate_output_amount(&price, input_amount).unwrap();

        // 计算步骤：
        // 1. 基础计算：1_000_000_000_000_000_000 * 3 / 1 = 3_000_000_000_000_000_000
        // 2. 精度调整：精度相同，不需要调整
        assert_eq!(output, 3_000_000_000_000_000_000);
    }

    #[test]
    fn test_output_higher_decimals() {
        let price = PairRate::new(
            ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            (1, 2),
            (18, 24),
        )
        .unwrap();

        // 输入1个token（18位精度）
        let input_amount = 1_000_000_000_000_000_000u128; // 1 token
        let output = PairRate::calculate_output_amount(&price, input_amount).unwrap();

        // 计算步骤：
        // 1. 基础计算：1_000_000_000_000_000_000 * 2 / 1 = 2_000_000_000_000_000_000
        // 2. 精度调整：从18位调整到24位，乘以10^6 = 2_000_000_000_000_000_000_000_000
        assert_eq!(output, 2_000_000_000_000_000_000_000_000);
    }

    #[test]
    fn test_adjust_decimals() {
        // 测试精度调整函数

        // 从高精度到低精度
        let result = PairRate::adjust_decimals(1_000_000_000_000_000_000_000_000, 24, 18).unwrap();
        assert_eq!(result, 1_000_000_000_000_000_000);

        // 从低精度到高精度
        let result = PairRate::adjust_decimals(1_000_000_000_000_000_000, 18, 24).unwrap();
        assert_eq!(result, 1_000_000_000_000_000_000_000_000);

        // 相同精度
        let result = PairRate::adjust_decimals(1_000_000_000_000_000_000, 18, 18).unwrap();
        assert_eq!(result, 1_000_000_000_000_000_000);
    }

    #[test]
    fn test_price_consistency() {
        let price = PairRate {
            token_pair: ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            rate: (3, 5), // 3 input_token = 5 output_token
            decimals: (24, 18),
        };

        // 测试往返计算的一致性
        let input_amount = 3_000_000_000_000_000_000_000_000u128; // 3 tokens with 24 decimals
        let output = PairRate::calculate_output_amount(&price, input_amount).unwrap();
        let back_to_input = PairRate::calculate_input_amount(&price, output).unwrap();

        assert_eq!(back_to_input, input_amount);
    }

    #[test]
    fn test_error_handling() {
        let price = PairRate {
            token_pair: ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            rate: (1, 1),
            decimals: (18, 18),
        };

        // 测试输入金额为0
        let result = PairRate::calculate_output_amount(&price, 0);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Input amount must be greater than 0".to_string()
        );

        // 测试输出金额为0
        let result = PairRate::calculate_input_amount(&price, 0);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Output amount must be greater than 0".to_string()
        );

        // 测试汇率为0
        let invalid_price = PairRate {
            token_pair: ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            rate: (0, 1),
            decimals: (18, 18),
        };
        let result = PairRate::calculate_output_amount(&invalid_price, 1000);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Rate components must be greater than 0".to_string()
        );
    }

    #[test]
    fn test_edge_cases() {
        let price = PairRate {
            token_pair: ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            rate: (1, 1),
            decimals: (18, 18),
        };

        // 测试非常大的数字（接近 u128 上限）
        let large_number = u128::MAX / 2;
        let result = PairRate::calculate_output_amount(&price, large_number);
        assert!(result.is_ok());

        // 测试非常小的数字
        let small_number = 1u128;
        let result = PairRate::calculate_output_amount(&price, small_number);
        assert!(result.is_ok());

        // 测试精度差异较大的情况
        let price_high_precision = PairRate {
            token_pair: ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            rate: (1, 1),
            decimals: (6, 24),
        };

        // 输入 1.0 token（6位精度）
        let input = 1_000_000u128; // 1.0 with 6 decimals
        let result = PairRate::calculate_output_amount(&price_high_precision, input);
        assert!(result.is_ok());

        // 期望输出：1.0 token（24位精度）
        // 1_000_000 * 10^(24-6) = 1_000_000 * 10^18
        let expected = input * 10u128.pow(18); // 将6位精度转换为24位精度
        assert_eq!(result.unwrap(), expected);

        // 测试反向转换
        let result = PairRate::calculate_input_amount(&price_high_precision, expected);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), input); // 应该回到原始输入值
    }

    #[test]
    fn test_helper_functions() {
        // 测试 is_valid
        let valid_price = PairRate::new(
            ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            (1, 1),
            (18, 18),
        )
        .unwrap();
        assert!(valid_price.is_valid());

        // 测试无效汇率
        let invalid_result = PairRate::new(
            ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            (0, 1),
            (18, 18),
        );
        assert!(invalid_result.is_err());

        // 测试超出最大汇率
        let invalid_rate_result = PairRate::new(
            ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            (MAX_RATE + 1, 1),
            (18, 18),
        );
        assert!(invalid_rate_result.is_err());

        // 测试超出最大精度
        let invalid_decimals_result = PairRate::new(
            ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            (1, 1),
            (MAX_DECIMALS + 1, 18),
        );
        assert!(invalid_decimals_result.is_err());

        // 测试 get_price_rate
        let price = PairRate::new(
            ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            (2, 5),
            (18, 18),
        )
        .unwrap();
        let ((rate_in, rate_out), (dec_in, dec_out)) = price.get_price_rate();
        assert_eq!(rate_in, 2);
        assert_eq!(rate_out, 5);
        assert_eq!(dec_in, 18);
        assert_eq!(dec_out, 18);

        // 测试 human_readable_rate
        // 1. 相同精度测试
        let price1 = PairRate::new(
            ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            (2, 5),
            (18, 18),
        )
        .unwrap();
        assert_eq!(price1.get_human_readable_rate(), 2.5);

        // 2. 不同精度测试 (input_decimals < output_decimals)
        let price2 = PairRate::new(
            ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            (1, 1),
            (6, 9),
        )
        .unwrap();
        assert_eq!(price2.get_human_readable_rate(), 0.001); // 1 * 10^-3

        // 3. 不同精度测试 (input_decimals > output_decimals)
        let price3 = PairRate::new(
            ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            (1, 1),
            (9, 6),
        )
        .unwrap();
        assert_eq!(price3.get_human_readable_rate(), 1000.0); // 1 * 10^3

        // 测试默认值
        let default_price = PairRate::default();
        assert_eq!(default_price.rate, (1, 1));
        assert_eq!(default_price.decimals, (18, 18));
        assert_eq!(default_price.token_pair.0, "TOKEN_A");
        assert_eq!(default_price.token_pair.1, "TOKEN_B");
        assert!(default_price.is_valid());
    }

    #[test]
    fn test_overflow_handling() {
        let price = PairRate {
            token_pair: ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            rate: (1, u128::MAX),
            decimals: (18, 18),
        };

        // 测试乘法溢出
        let result = PairRate::calculate_output_amount(&price, u128::MAX);
        assert!(result.is_err());

        // 测试精度调整溢出
        let price_high_precision = PairRate {
            token_pair: ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            rate: (1, 1),
            decimals: (0, 38), // 超过最大安全精度
        };
        let result = PairRate::calculate_output_amount(&price_high_precision, u128::MAX / 2);
        assert!(result.is_err());
    }
}
