
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