
/// 给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。
/// 
/// 示例 1:
/// 
/// 输入: 123
/// 输出: 321
/// 
///  示例 2:
/// 
/// 输入: -123
/// 输出: -321
/// 
/// 示例 3:
/// 
/// 输入: 120
/// 输出: 21
/// 
/// 注意:
/// 
/// 假设我们的环境只能存储得下 32 位的有符号整数，则其数值范围为 [−231,  231 − 1]。请根据这个假设，如果反转后整数溢出那么就返回 0。

pub fn reverse(x: i32) -> i32
{
    let mut y = 0 as i64;
    let mut xx = x as i64;
    
    while xx != 0
    {
    	y = (xx % 10) + y * 10;
    	xx = xx / 10;
    }
    
    if y <= i32::max_value() as i64 && y >= i32::min_value() as i64
    {
    	return y as i32;
    }
    
    0
}

#[cfg(test)]
mod test
{
    use super::reverse;

    #[test]
    fn test_reverse()
    {
        assert_eq!(reverse(123), 321);
    }
}