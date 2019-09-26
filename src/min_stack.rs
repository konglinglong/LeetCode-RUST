/// 设计一个支持 push，pop，top 操作，并能在常数时间内检索到最小元素的栈。
/// 
/// push(x) -- 将元素 x 推入栈中。
/// pop() -- 删除栈顶的元素。
/// top() -- 获取栈顶元素。
/// getMin() -- 检索栈中的最小元素。
/// 
/// 示例:
/// 
/// MinStack minStack = new MinStack();
/// minStack.push(-2);
/// minStack.push(0);
/// minStack.push(-3);
/// minStack.getMin();   --> 返回 -3.
/// minStack.pop();
/// minStack.top();      --> 返回 0.
/// minStack.getMin();   --> 返回 -2.

#[allow(dead_code, non_snake_case)]
#[derive(Debug)]
struct StackValue {
    value: i32,
    minValue: i32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct MinStack {
    stack: Vec<StackValue>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

#[allow(dead_code)]
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            stack: Vec::<StackValue>::new(),
        }
    }

    fn push(&mut self, x: i32) {
        let min;
        if self.stack.len() > 0 && self.stack[self.stack.len() - 1].minValue < x {
            min = self.stack[self.stack.len() - 1].minValue;
        } else {
            min = x;
        }
        self.stack.push(StackValue{value: x, minValue: min});
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&mut self) -> i32 {
        if self.stack.len() > 0 {
            self.stack[self.stack.len() - 1].value
        } else {
            0
        }
    }

    fn get_min(&self) -> i32 {
        if self.stack.len() > 0 {
            self.stack[self.stack.len() - 1].minValue
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_min_stack()
    {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        println!("stack {:?}", stack);
//        assert_eq!(compare_version("0.1".to_string(), "1.1".to_string()), -1);
//        assert_eq!(compare_version("1.0.1".to_string(), "1.0".to_string()), 1);
    }
}
