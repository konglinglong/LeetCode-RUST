/// ```rust,no_run
/// 给定一个正整数，返回它在 Excel 表中相对应的列名称。
///
/// 例如，
///
///     1 -> A
///     2 -> B
///     3 -> C
///     ...
///     26 -> Z
///     27 -> AA
///     28 -> AB
///     ...
///
/// 示例 1:
///
/// 输入: 1
/// 输出: "A"
///
/// 示例 2:
///
/// 输入: 28
/// 输出: "AB"
///
/// 示例 3:
///
/// 输入: 701
/// 输出: "ZY"
/// 
/// ```

use std::collections::HashMap;

pub fn convert_to_title(n: i32) -> String {
	let mut v = n;
	let mut char_vector: Vec<char> = Vec::new();
	let ch_map: HashMap<i32, char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
		.chars()
		.enumerate()
		.map(|(x, y)| (x as i32, y))
		.collect();
	loop {
		char_vector.insert(0, ch_map[&((v - 1) % 26)]);
		v = (v - 1) / 26;
		if v == 0 {
			break;
		}
	}

	println!("{:?}", char_vector);
	char_vector.iter().cloned().collect::<String>()
}

#[cfg(test)]
mod test {
	use super::convert_to_title;

	#[test]
	fn test_convert_to_title() {
		assert_eq!(convert_to_title(1), "A".to_string());
		assert_eq!(convert_to_title(28), "AB".to_string());
	}
}
