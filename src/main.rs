use std::cmp::Ordering::*;

/*
 * my own generic binary search implementation... just for fun lol
 */
fn binary_search<T: Ord>(arr: &[T], value: &T) -> Option<usize> {
    let mut start = 0;
    let mut end = arr.len();

    while end - start != 1 {
        let firsthalf_end = start + (end - start) / 2;
        match value.cmp(&arr[firsthalf_end]) {
            Less => {
                end = firsthalf_end;
            }

            Equal => {
                return Some(firsthalf_end);
            }

            Greater => {
                start = firsthalf_end;
            }
        }
    }

    if arr[start] == *value {
        Some(start)
    } else {
        None
    }
}

fn main() {
    assert_eq!(binary_search(&[2, 23, 52, 55], &52), Some(2));
}
