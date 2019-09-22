/// ```rust,ignore
/// 171. Excel表列序号
/// 给定一个Excel表格中的列名称，返回其相应的列序号。
///
/// 例如，
///
/// A -> 1
/// B -> 2
/// C -> 3
/// ...
/// Z -> 26
/// AA -> 27
/// AB -> 28
/// ...
///
/// 示例 1:
///
/// 输入: "A"
/// 输出: 1
///
/// 示例 2:
///
/// 输入: "AB"
/// 输出: 28
///
/// 示例 3:
///
/// 输入: "ZY"
/// 输出: 701
///
/// 致谢：
/// 特别感谢 @ts 添加此问题并创建所有测试用例。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/excel-sheet-column-number
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
/// ```

pub fn title_to_number(s: String) -> i32 {
    let bytes = s.into_bytes();
    let mut val = 0 as i32;
    for i in bytes {
        val = val * 26 - ('A' as i32) + (i as i32) + 1;
    }
    val as i32
}

#[cfg(test)]
mod test
{
    use super::title_to_number;

    #[test]
    fn test_title_to_number()
    {
        assert_eq!(title_to_number("A".to_string()), 1);
    }
}

