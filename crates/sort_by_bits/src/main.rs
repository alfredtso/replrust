fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_by(|x, y| {
            // Create a tuple of 1s and the value itself
            let x = (x.count_ones(), *x);
            let y = (y.count_ones(), *y);
            // The comparison will compare first the number of 1s, then the value itself
            x.cmp(&y)
        });
        arr
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let arr = vec![0,1,2,3,4,5,6,7,8];
        let res = Solution::sort_by_bits(arr);
        assert_eq!(res, vec![0,1,2,4,8,3,5,6,7]);
    }

}


