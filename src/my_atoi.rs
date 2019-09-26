/// 
/// 请你来实现一个 atoi 函数，使其能将字符串转换成整数。
/// 
/// 首先，该函数会根据需要丢弃无用的开头空格字符，直到寻找到第一个非空格的字符为止。
/// 
/// 当我们寻找到的第一个非空字符为正或者负号时，则将该符号与之后面尽可能多的连续数字组合起来，作为该整数的正负号；假如第一个非空字符是数字，则直接将其与之后连续的数字字符组合起来，形成整数。
/// 
/// 该字符串除了有效的整数部分之后也可能会存在多余的字符，这些字符可以被忽略，它们对于函数不应该造成影响。
/// 
/// 注意：假如该字符串中的第一个非空格字符不是一个有效整数字符、字符串为空或字符串仅包含空白字符时，则你的函数不需要进行转换。
/// 
/// 在任何情况下，若函数不能进行有效的转换时，请返回 0。
/// 
/// 说明：
/// 
/// 假设我们的环境只能存储 32 位大小的有符号整数，那么其数值范围为 [−231,  231 − 1]。如果数值超过这个范围，qing返回  INT_MAX (231 − 1) 或 INT_MIN (−231) 。
/// 
/// 示例 1:
/// 
/// 输入: "42"
/// 输出: 42
/// 
/// 示例 2:
/// 
/// 输入: "   -42"
/// 输出: -42
/// 解释: 第一个非空白字符为 '-', 它是一个负号。
///      我们尽可能将负号与后面所有连续出现的数字组合起来，最后得到 -42 。
/// 
/// 示例 3:
/// 
/// 输入: "4193 with words"
/// 输出: 4193
/// 解释: 转换截止于数字 '3' ，因为它的下一个字符不为数字。
/// 
/// 示例 4:
/// 
/// 输入: "words and 987"
/// 输出: 0
/// 解释: 第一个非空字符是 'w', 但它不是数字或正、负号。
///      因此无法执行有效的转换。
/// 
/// 示例 5:
/// 
/// 输入: "-91283472332"
/// 输出: -2147483648
/// 解释: 数字 "-91283472332" 超过 32 位有符号整数范围。 
///      因此返回 INT_MIN (−231) 。
/// 

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum State {
	Init,
	Sign,
	NumBeg,
}

pub fn my_atoi(str: String) -> i32 {
	let mut state = State::Init;
	let mut beg_pos = 0;
	let mut end_pos = 0;
	
	let mut s = str.trim().to_string();
	
	for (i, c) in s.char_indices() {
		match c {
			'0'..='9' => {
				if state == State::Init {
					state = State::NumBeg;
					beg_pos = i;
					end_pos = i + 1;
				}
				else if state == State::Sign {
					state = State::NumBeg;
					end_pos = i + 1;
				} else if state == State::NumBeg {
					end_pos = i + 1;
				} else {
					return 0;
				}
			}
			'-'=> {
				if state == State::Init {
					state = State::Sign;
					beg_pos = i;
					end_pos = i + 1;
				} else if state == State::NumBeg {
					end_pos = i;
					break
				}
				else {
					return 0;
				}
			}
			'+' => {
				if state == State::Init {
					state = State::Sign;
					beg_pos = i;
					end_pos = i + 1;
				} else if state == State::NumBeg {
					end_pos = i;
					break
				}
				else {
					return 0;
				}
			}
			_ => {
				if state == State::NumBeg {
					end_pos = i;
					break
				}
				else {
					return 0;
				}
			}
		}
	}
	
	s.split_off(end_pos);
	s = s.split_off(beg_pos);
	
//	println!("beg_pos = {}, end_pos = {}, s = {}", beg_pos, end_pos, s);
	
	if s.len() == 0 {
		return 0;
	}

	let overflow = "1231123123123123".parse::<u32>().err().unwrap();
	let underflow = "-1231123123123123".parse::<i32>().err().unwrap();

//    println!("min_value = {}, max_value = {}", i32::min_value(), i32::max_value());

    let result = match s.parse::<i32>() {
        Ok(result) => result,
        Err(ref e) if *e == overflow => i32::max_value(),
        Err(ref e) if *e == underflow => i32::min_value(),
        _ => {0},
    };
	
//	println!("result = {}", result);
	
    result as i32
}

#[cfg(test)]
mod test
{
    use super::my_atoi;

    #[test]
    fn test_my_atoi()
    {
        assert_eq!(my_atoi("-5-".to_string()), -5);
        assert_eq!(my_atoi("+".to_string()), 0);
        assert_eq!(my_atoi("+1".to_string()), 1);
        assert_eq!(my_atoi("42".to_string()), 42);
        assert_eq!(my_atoi("9223372036854775808".to_string()), 2147483647);
        assert_eq!(my_atoi("-91283472332".to_string()), -2147483648);
        assert_eq!(my_atoi("3.14159".to_string()), 3);
        assert_eq!(my_atoi("4193 with words".to_string()), 4193);
    }
}