use jni::objects::AutoElements;

// TODO: Ensure that we are doing bubble sort the most optimally
pub fn bubble_sort(arr: &mut AutoElements<i32>) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                let curr = arr[j];
                let prev = arr[j + 1];
                arr[j] = prev;
                arr[j + 1] = curr;
            }
        }
    }
}

pub fn merge_sort(arr: &mut AutoElements<i32>) {
    merge_sort::sort(arr, 0, (arr.len() - 1) as isize)
}

mod merge_sort {
    use jni::objects::AutoElements;

    pub fn sort(arr: &mut AutoElements<i32>, left: isize, right: isize) {
        if left > right {
            return;
        }
        let middle = left + (right - left) / 2;
        sort(arr, left, middle - 1);
        sort(arr, middle + 1, right);
        merge(arr, left as usize, middle as usize, right as usize)
    }

    fn merge(arr: &mut AutoElements<i32>, l: usize, m: usize, r: usize) {
        let left_amount = m - l + 1;
        let right_amount = r - m;
        let mut left: Vec<i32> = vec![0; left_amount];
        let mut right: Vec<i32> = vec![0; right_amount];
        for i in 0..(left_amount) {
            left[i] = arr[l + i];
        }
        for i in 0..right_amount {
            right[i] = arr[m + 1 + i];
        }

        // keep checking against left vs right.
        // if one is smaller we swap them and increment the counter for the found side
        let mut r_index = 0;
        let mut l_index = 0;
        let mut curr_index = l;
        let mut x = 0;
        while r_index < right_amount && l_index < left_amount {
            if left[l_index] <= right[r_index] {
                arr[curr_index] = left[l_index];
                l_index += 1;
            } else {
                arr[curr_index] = right[r_index];
                r_index += 1;
            }
            curr_index += 1;
            x += 1;
            if x == 1000 {
                break;
            }
        }

        // insert the remaining from left and right
        while l_index < left_amount {
            arr[curr_index]= left[l_index];
            l_index += 1;
            curr_index += 1;
        }

        while r_index < right_amount {
            arr[curr_index]= right[r_index];
            r_index += 1;
            curr_index += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use jni::{objects::ReleaseMode, InitArgsBuilder, JNIVersion, JavaVM};
    use lazy_static::lazy_static;

    use super::*;

    lazy_static! {
        static ref VM: JavaVM = {
            let args = InitArgsBuilder::new()
                .version(JNIVersion::V8)
                .build()
                .unwrap();
            JavaVM::new(args).unwrap()
        };
    }

    #[test]
    fn can_bubble_sort() {
        test_sort(bubble_sort)
    }

    #[test]
    fn can_merge_sort() {
        test_sort(merge_sort)
    }

    fn test_sort(sort: fn(&mut AutoElements<i32>)) {
        let mut env = VM.attach_current_thread_as_daemon().unwrap();
        if let Ok(arr) = env.new_int_array(5) {
            unsafe {
                if let Ok(mut arr) = env.get_array_elements(&arr, ReleaseMode::CopyBack) {
                    arr[0] = 5;
                    arr[1] = 2;
                    arr[2] = 10;
                    arr[3] = 0;
                    arr[4] = -2;
                    sort(&mut arr);
                    assert!(
                        arr[0] == -2 && arr[1] == 0 && arr[2] == 2 && arr[3] == 5 && arr[4] == 10
                    );
                }
            }
        }
    }
}
