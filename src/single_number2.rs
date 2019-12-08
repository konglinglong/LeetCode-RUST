/// ```rust,ignore
/// 137. 只出现一次的数字 II
/// 给定一个非空整数数组，除了某个元素只出现一次以外，其余每个元素均出现了三次。找出那个只出现了一次的元素。
///
/// 说明：
///
/// 你的算法应该具有线性时间复杂度。 你可以不使用额外空间来实现吗？
///
/// 示例 1:
///
/// 输入: [2,2,3,2]
/// 输出: 3
///
/// 示例 2:
///
/// 输入: [0,1,0,1,0,1,99]
/// 输出: 99
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/single-number-ii
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
/// ```

pub fn single_number2(nums: Vec<i32>) -> i32 {
    let (mut a, mut b) = (0, 0);
    for i in nums {
        b = (b ^ i) & (!a);
        a = (a ^ i) & (!b);
    }
    b
}


#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_single_number2()
    {
        assert_eq!(single_number2(vec![0,1,0,1,0,1,99]), 99);
    }
}
