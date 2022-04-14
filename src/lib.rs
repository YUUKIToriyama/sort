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

    /// シェーカーソート
    pub fn shaker_sort(mut arr: Vec<i32>) -> Vec<i32> {
        let l = arr.len() - 1;
        for i in 0..l {
            for j in i..l {
                if i % 2 == 0 {
                    let a = arr[j];
                    let b = arr[j + 1];
                    if a > b {
                        arr[j + 1] = a;
                        arr[j] = b;
                    }
                } else {
                    let c = arr[l - j - 1];
                    let d = arr[l - j];
                    if c > d {
                        arr[l - j] = c;
                        arr[l - j - 1] = d;
                    }
                }
            }
        }
        return arr;
    }

    /// コムソート
    pub fn comb_sort(mut arr: Vec<i32>) -> Vec<i32> {
        let mut h = arr.len() * 10 / 13;
        loop {
            let mut swap_count = 0;
            let mut i = 0;
            while i + h < arr.len() {
                let a = arr[i];
                let b = arr[i + h];
                if a > b {
                    arr[i + h] = a;
                    arr[i] = b;
                    swap_count = swap_count + 1;
                }
                i = i + 1;
            }
            if h == 1 {
                if swap_count == 0 {
                    break;
                }
            } else {
                h = h * 10 / 13;
            }
        }
        return arr;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cases() -> Vec<(Vec<i32>, Vec<i32>)> {
        return vec![
            (vec![1, 2, 3], vec![1, 2, 3]),
            (vec![9, 5, 1], vec![1, 5, 9]),
            (vec![99, 33, 55, 11], vec![11, 33, 55, 99]),
            (
                vec![1, 9, 2, 8, 3, 7, 4, 6, 5],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            ),
        ];
    }

    #[test]
    fn test_bubble_sort() {
        for test_case in test_cases() {
            assert_eq!(sort::bubble_sort(test_case.0), test_case.1);
        }
    }

    #[test]
    fn test_shaker_sort() {
        for test_case in test_cases() {
            assert_eq!(sort::shaker_sort(test_case.0), test_case.1);
        }
    }

    #[test]
    fn test_comb_sort() {
        for test_case in test_cases() {
            assert_eq!(sort::comb_sort(test_case.0), test_case.1);
        }
    }
}
