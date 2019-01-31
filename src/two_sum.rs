
/// 给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
/// 
/// 你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。
/// 
/// 示例:
/// 
/// 给定 nums = [2, 7, 11, 15], target = 9
/// 
/// 因为 nums[0] + nums[1] = 2 + 7 = 9
/// 所以返回 [0, 1]

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>
{
	let n = nums.len();
	for i in 0..n
	{
		for ii in (i + 1)..n
		{
			if nums[i] + nums[ii] == target
			{
				return vec![i as i32, ii as i32];
			}
		}
	}
    vec![]
}


#[cfg(test)]
mod test
{
	use super::two_sum;
	
	#[test]
	fn test_tow_sum()
	{
		assert_eq!(two_sum(vec![2,7,11,15], 9), vec![0, 1]);
		assert_eq!(two_sum(vec![2,5,5,11], 10), vec![1, 2]);
	}
}