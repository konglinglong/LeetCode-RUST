/// 给定一个大小为 n 的数组，找到其中的众数。众数是指在数组中出现次数大于 ⌊ n/2 ⌋ 的元素。
/// 
/// 你可以假设数组是非空的，并且给定的数组总是存在众数。
/// 
/// 示例 1:
/// 
/// 输入: [3,2,3]
/// 输出: 3
/// 
/// 示例 2:
/// 
/// 输入: [2,2,1,1,1,2,2]
/// 输出: 2

use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let n = ((nums.len() + 1) / 2) as i32;
    let mut map: HashMap<i32, i32> = HashMap::new();
    nums.iter().for_each(|x| {map.entry(*x).and_modify(|e| { *e += 1 }).or_insert(1);});
    
//    println!("{:?}", map);
    
    for (key, val) in map {
    	if val >= n {
    		return key;
    	}
    }
    
    0
}

pub fn majority_element_2(nums: Vec<i32>) -> i32 {
    let n = ((nums.len() + 1) / 2) as i32;
    let mut vote: i32 = 0;
    let mut count: i32 = 0;
    nums.iter().for_each(|x| {
    		if count == 0 {
    			vote = *x;
    			count = 1;
    		} else {
    			if vote == *x {
    				count += 1;
    			} else {
    				count -= 1;
    			}
    		}
    });
    
    println!("vote = {}", vote);
    
    count = 0;
    nums.iter().for_each(|x| {
    		if *x == vote {
    			count += 1;
    		}
    });
    
    if count >= n {
    	return vote;
    }
    
    0
}

#[cfg(test)]
mod test
{
    use super::majority_element;
    use super::majority_element_2;

    #[test]
    fn test_majority_element()
    {
        assert_eq!(majority_element(vec![3,2,3]), 3);
        assert_eq!(majority_element_2(vec![3,2,3]), 3);
        assert_eq!(majority_element(vec![2,2,1,1,1,2,2]), 2);
        assert_eq!(majority_element_2(vec![2,2,1,1,1,2,2]), 2);
    }
}