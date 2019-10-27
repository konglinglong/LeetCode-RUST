/// ```rust,ignore
/// 118. 杨辉三角
/// 给定一个非负整数 numRows，生成杨辉三角的前 numRows 行。
///
/// 在杨辉三角中，每个数是它左上方和右上方的数的和。
///
/// 示例:
///
/// 输入: 5
/// 输出:
/// [
/// [1],
/// [1,1],
/// [1,2,1],
/// [1,3,3,1],
/// [1,4,6,4,1]
/// ]
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/pascals-triangle
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
/// ```

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut vec:Vec<Vec<i32>> = Vec::new();
    for i in 0..num_rows {
        vec.push(vec![1; (i + 1) as usize]);
        if i > 1 {
            for j in 1..i {
                vec[i as usize][j as usize] = vec[(i - 1) as usize][(j - 1) as usize] + vec[(i - 1) as usize][j as usize];
            }
        }
    }
    vec
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_generate()
    {
        assert_eq!(generate(2), vec![vec![1], vec![1,1]]);
    }
}
