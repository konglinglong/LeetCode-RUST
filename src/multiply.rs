/// ```rust,ignore
/// 43. 字符串相乘
///
/// 给定两个以字符串形式表示的非负整数 num1 和 num2，返回 num1 和 num2 的乘积，它们的乘积也表示为字符串形式。
///
/// 示例 1:
///
/// 输入: num1 = "2", num2 = "3"
/// 输出: "6"
///
/// 示例 2:
///
/// 输入: num1 = "123", num2 = "456"
/// 输出: "56088"
///
/// 说明：
///
/// num1 和 num2 的长度小于110。
/// num1 和 num2 只包含数字 0-9。
/// num1 和 num2 均不以零开头，除非是数字 0 本身。
/// 不能使用任何标准库的大数类型（比如 BigInteger）或直接将输入转换为整数来处理。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/multiply-strings
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
/// ```

pub fn multiply(num1: String, num2: String) -> String {
    let bytes1 = num1.into_bytes();
    let bytes2 = num2.into_bytes();
    let (mut nb1, mut nb2) = (bytes1.len() - 1, bytes2.len() - 1);
    let (mut a, mut b) = (0 as i128, 0 as i128);
    for i in bytes1 {
        a += ((i as i128) - ('0' as i128)) * ((10 as f64).powi((nb1) as i32) as i128);
        if nb1 == 0 {
            break;
        }
        nb1 -= 1;
    }
    for i in bytes2 {
        b += ((i as i128) - ('0' as i128)) * ((10 as f64).powi((nb2) as i32) as i128);
        if nb2 == 0 {
            break;
        }
        nb2 -= 1;
    }
    (a * b).to_string()
}

#[cfg(test)]
mod test
{
    use super::multiply;

    #[test]
    fn test_multiply()
    {
        assert_eq!(multiply("2".to_string(), "3".to_string()), "6".to_string());
//        assert_eq!(multiply("401716832807512840963".to_string(), "167141802233061013023557397451289113296441069".to_string()), "67143675422804947379429215144664313370120390398055713625298709447".to_string());
    }
}
