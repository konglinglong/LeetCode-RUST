/// ```rust,ignore
/// 172. 阶乘后的零
///
/// 给定一个整数 n，返回 n! 结果尾数中零的数量。
///
/// 示例 1:
///
/// 输入: 3
/// 输出: 0
/// 解释: 3! = 6, 尾数中没有零。
///
/// 示例 2:
///
/// 输入: 5
/// 输出: 1
/// 解释: 5! = 120, 尾数中有 1 个零.
///
/// 说明: 你算法的时间复杂度应为 O(log n) 。
/// ```

pub fn trailing_zeroes(n: i32) -> i32 {
    let mut n = n;
    let mut count = 0;
    while n >= 5 {
        count += n / 5;
        n /= 5;
    }
    count
}

#[cfg(test)]
mod test
{
    use super::trailing_zeroes;

    #[test]
    fn test_trailing_zeroes()
    {
        assert_eq!(trailing_zeroes(3), 0);
    }
}
