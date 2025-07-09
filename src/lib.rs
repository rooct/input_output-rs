use num_bigint::BigUint;
use num_traits::ToPrimitive;

/// 价格信息结构体
#[derive(Debug, Clone, PartialEq)]
pub struct PairRate {
    pub token_pair: (String, String),
    pub rate: (u128, u128),
    pub decimals: (u8, u8),
}

impl PairRate {
    /// 根据价格和输入代币数量计算输出代币数量
    /// price: 价格信息
    /// input_amount: 输入代币数量（已经按照input_decimals格式化）
    pub fn calculate_output_amount(price: &PairRate, input_amount: u128) -> Result<u128, String> {
        if input_amount == 0 {
            return Err("Input amount must be greater than 0".to_string());
        }

        if price.rate.0 == 0 || price.rate.1 == 0 {
            return Err("PairRate components must be greater than 0".to_string());
        }

        let (input_rate, output_rate) = price.rate;

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
    /// price: 价格信息
    /// output_amount: 期望的输出代币数量（已经按照output_decimals格式化）
    pub fn calculate_input_amount(price: &PairRate, output_amount: u128) -> Result<u128, String> {
        if output_amount == 0 {
            return Err("Output amount must be greater than 0".to_string());
        }

        if price.rate.0 == 0 || price.rate.1 == 0 {
            return Err("PairRate components must be greater than 0".to_string());
        }

        let (input_rate, output_rate) = price.rate;

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
    /// amount: 要调整的数量
    /// from_decimals: 原始精度
    /// to_decimals: 目标精度
    fn adjust_decimals(amount: u128, from_decimals: u8, to_decimals: u8) -> Result<u128, String> {
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

        let amount = BigUint::from(amount);
        let multiplier = BigUint::from(multiplier);
        let divisor = BigUint::from(divisor);

        let result = amount * multiplier / divisor;

        result.to_u128().ok_or("Result exceeds u128".to_string())
    }

    /// 计算最大公约数
    fn gcd(mut a: u128, mut b: u128) -> u128 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    /// 获取当前价格率（输入代币相对于输出代币的价格）
    pub fn get_price_rate(&self) -> f64 {
        let (input_rate, output_rate) = self.rate;
        output_rate as f64 / input_rate as f64
    }

    /// 验证价格是否有效
    pub fn is_valid(&self) -> bool {
        self.rate.0 > 0 && self.rate.1 > 0
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
        let price = PairRate {
            token_pair: ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            rate: (10, 19), // 1 input_token = 2 output_token
            decimals: (24, 24),
        };

        // 输入1个token（24位精度）
        let input_amount = 200_000_000_000_000_000_000_000_000u128; // 1 token with 24 decimals
        let output = PairRate::calculate_output_amount(&price, input_amount).unwrap();
        println!("Output amount: {}", output);
    }

    #[test]
    fn test_calculate_output_amount_r() {
        let price = PairRate {
            token_pair: ("TOKEN_B".to_string(), "TOKEN_A".to_string()),
            rate: (19, 10), // 1 input_token = 2 output_token
            decimals: (24, 24),
        };

        // 输入1个token（24位精度）
        let input_amount = 200_000_000_000_000_000_000_000_000u128; // 1 token with 24 decimals
        let output = PairRate::calculate_output_amount(&price, input_amount).unwrap();
        println!("Output amount: {}", output);
    }

    #[test]
    fn test_calculate_input_amount() {
        let price = PairRate {
            token_pair: ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            rate: (1, 2), // 1 input_token = 2 output_token
            decimals: (24, 18),
        };

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
        let price = PairRate {
            token_pair: ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            rate: (1, 3), // 1 input_token = 3 output_token
            decimals: (18, 18),
        };

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
        let price = PairRate {
            token_pair: ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            rate: (1, 2), // 1 input_token = 2 output_token
            decimals: (18, 24),
        };

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
            "PairRate components must be greater than 0".to_string()
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
        let valid_price = PairRate {
            token_pair: ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            rate: (1, 1),
            decimals: (18, 18),
        };
        assert!(valid_price.is_valid());

        let invalid_price = PairRate {
            token_pair: ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            rate: (0, 1),
            decimals: (18, 18),
        };
        assert!(!invalid_price.is_valid());

        // 测试 get_price_rate
        let price = PairRate {
            token_pair: ("TOKEN_A".to_string(), "TOKEN_B".to_string()),
            rate: (2, 5),
            decimals: (18, 18),
        };
        assert_eq!(price.get_price_rate(), 2.5);

        // 测试默认值
        let default_price = PairRate::default();
        assert_eq!(default_price.rate, (1, 1));
        assert_eq!(default_price.decimals, (18, 18));
        assert_eq!(default_price.token_pair.0, "TOKEN_A");
        assert_eq!(default_price.token_pair.1, "TOKEN_B");
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
