pub fn bubble_sort<T>(a: &mut [T])
where
    T: PartialOrd,
{
    for i in 1..a.len() {
        let mut ok = true;

        for j in 0..(a.len() - i) {
            if a[j] > a[j + 1] {
                ok = false;
                a.swap(j, j + 1);
            }
        }

        if ok {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest(input, expected,
    case(vec![], vec![]),
    case(vec![1], vec![1]),
    case(vec![0, 0, 0], vec![0, 0, 0]),
    case(vec![1, 2, 3], vec![1, 2, 3]),
    case(vec![3, 2, 1], vec![1, 2, 3]),
    case(vec![1, 3, 2], vec![1, 2, 3]),
    )]
    fn test_bubble_sort(mut input: Vec<u8>, expected: Vec<u8>) {
        bubble_sort(&mut input);
        assert_eq!(input, expected)
    }
}
