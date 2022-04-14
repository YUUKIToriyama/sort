pub mod sort {
    //! sorting module
    /// バブルソート
    pub fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
        for i in 0..arr.len() - 1 {
            for j in 1..(arr.len() - i) {
                let a = arr[j - 1];
                let b = arr[j];
                if a > b {
                    arr[j - 1] = b;
                    arr[j] = a;
                }
            }
        }
        return arr;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort() {
        assert_eq!(sort::bubble_sort(vec![1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(sort::bubble_sort(vec![9, 5, 1]), vec![1, 5, 9]);
        assert_eq!(sort::bubble_sort(vec![99, 33, 55]), vec![33, 55, 99]);
        assert_eq!(
            sort::bubble_sort(vec![1, 9, 2, 8, 3, 7, 4, 6, 5]),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }
}
