struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m as usize;
        let mut n = n as usize;

        while n > 0 {
            if m > 0 && nums1[m-1] > nums2[n-1] {
                nums1[m+n-1] = nums1[m-1];
                m -= 1;
            } else {
                nums1[m+n-1] = nums2[n-1];
                n -= 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let mut nums1 = vec![1,2,3,0,0,0];
        let m = 3;
        let mut nums2 = vec![2,5,6];
        let n = 3;
        let res = Solution::merge(&mut nums1,m,&mut nums2,n);
        assert_eq!(nums1, vec![1,2,2,3,5,6]);
    }

    #[test]
    fn test2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        let res = Solution::merge(&mut nums1,m,&mut nums2,n);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test3() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;
        let res = Solution::merge(&mut nums1,m,&mut nums2,n);
        assert_eq!(nums1, vec![1]);
    }
}


