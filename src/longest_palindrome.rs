/// ```rust,ignore
/// 给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。
///
/// 示例 1：
///
/// 输入: "babad"
/// 输出: "bab"
/// 注意: "aba" 也是一个有效答案。
///
/// 示例 2：
///
/// 输入: "cbbd"
/// 输出: "bb"
/// ```

pub fn longest_palindrome(s: String) -> String {
    let ss = s.as_bytes();
    let len = ss.len();
    if len <= 1 {
        return s;
    }

    let (mut begin, mut end, mut left, mut right, mut max) = (0, 0, 0, 0, 0);
    for mid in 0..len {
        left = mid;
        right = mid;
        while left > 0 && ss[left - 1] == ss[mid] {
            left -= 1;
        }
        while right < len - 1 && ss[right + 1] == ss[mid] {
            right += 1;
        }

        while left > 0 && right < len - 1 && ss[left - 1] == ss[right + 1] {
            left -= 1;
            right += 1;
        }
        if right - left > max {
            begin = left;
            end = right;
            max = end - begin;
        }
    }

    s[begin..end + 1].to_string()
}


#[cfg(test)]
mod test
{
    use super::longest_palindrome;

    #[test]
    fn test_longest_palindrome()
    {
        assert_eq!(longest_palindrome(String::from("babad")), String::from("bab"));
    }
}