/// 给定一个大小为 n 的数组，找出其中所有出现超过 ⌊ n/3 ⌋ 次的元素。
/// 
/// 说明: 要求算法的时间复杂度为 O(n)，空间复杂度为 O(1)。
/// 
/// 示例 1:
/// 
/// 输入: [3,2,3]
/// 输出: [3]
/// 
/// 示例 2:
/// 
/// 输入: [1,1,1,3,3,2,2,2]
/// 输出: [1,2]
/// 


const N: usize = 2;

pub fn have_zero_count(count: &[i32;N]) -> bool {
	if count.iter().find(|&&a| a == 0) == None {
		return false;
	}
	return true;
}

pub fn is_vote_exist(v: i32, vote: &[i32;N], count: &[i32;N]) -> bool {
	for i in 0..N {
		if vote[i] == v && count[i] != 0 {
			return true;
		}
	}
	return false;
}

pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
	
	println!("nums = {:?}", nums);
	
    let n = (nums.len() / (N + 1)) as i32;
    let mut v: Vec<i32> = Vec::new();
    let mut vote: [i32;N] = [0;N];
    let mut count: [i32;N] = [0;N];
    let mut count2: [i32;N] = [0;N];

    nums.iter().for_each(|&x| {
		let zero_c_exist = have_zero_count(&count);
		let v_exist = is_vote_exist(x, &vote, &count);
		if v_exist {
			for i in 0..N {
				if vote[i] == x {
					count[i] += 1;
					break;
				}
			}
		} else {
			if zero_c_exist {
				for i in 0..N {
					if count[i] == 0 {
		    			vote[i] = x;
		    			count[i] = 1;
		    			break;
					}
				}
			} else {
				for i in 0..N {
		    		count[i] -= 1;
				}
			}
		}
		println!("vote = {:?}, count = {:?}", vote, count);
    });
    
    println!("n = {}, vote = {:?}, count = {:?}", n, vote, count);
    
    nums.iter().for_each(|x| {
		for i in 0..N {
    		if *x == vote[i] && count[i] > 0 {
    			count2[i] += 1;
    		}
		}
    });
    
	for i in 0..N {
	    if count2[i] > n {
	    	v.push(vote[i]);
	    }
	}
	
	//println!("v = {:?}", v);
	
	v
}

#[cfg(test)]
mod test
{
    use super::majority_element;

    #[test]
    fn test_majority_element()
    {
        assert_eq!(majority_element(vec![0,0,0]), vec![0]);
        assert_eq!(majority_element(vec![3,2,3]), vec![3]);
        assert_eq!(majority_element(vec![1,1,1,3,3,2,2,2]), vec![1,2]);
    }
}
