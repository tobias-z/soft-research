use jni::objects::AutoElementsCritical;

// TODO: Ensure that we are doing bubble sort the most optimally
pub fn bubble_sort(arr: &mut AutoElementsCritical<i32>) {
    for _ in 0..arr.len() {
        for j in 0..arr.len() - 1 {
            if arr[j] > arr[j + 1] {
                let curr = arr[j];
                let prev = arr[j + 1];
                arr[j] = prev;
                arr[j + 1] = curr;
            }
        }
    }
}

pub fn merge_sort(arr: &mut AutoElementsCritical<i32>) {
    merge_sort::sort(arr, 0, (arr.len() - 1) as isize)
}

/// # Safety
///
/// This function uses pointer arithmatic to mutate the an array
pub unsafe fn merge_sort_ptr_arithmatic(arr: *mut i32, length: i32) {
    merge_sort::sort_ptr_arithmatic(arr, 0, (length - 1) as isize)
}

mod merge_sort {
    use jni::objects::AutoElementsCritical;

    pub fn sort(arr: &mut AutoElementsCritical<i32>, left: isize, right: isize) {
        if left >= right {
            return;
        }
        let middle = left + (right - left) / 2;
        sort(arr, left, middle);
        sort(arr, middle + 1, right);
        merge(arr, left as usize, middle as usize, right as usize)
    }

    fn merge(arr: &mut AutoElementsCritical<i32>, l: usize, m: usize, r: usize) {
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
        while r_index < right_amount && l_index < left_amount {
            if left[l_index] <= right[r_index] {
                arr[curr_index] = left[l_index];
                l_index += 1;
            } else {
                arr[curr_index] = right[r_index];
                r_index += 1;
            }
            curr_index += 1;
        }

        // insert the remaining from left and right
        while l_index < left_amount {
            arr[curr_index] = left[l_index];
            l_index += 1;
            curr_index += 1;
        }

        while r_index < right_amount {
            arr[curr_index] = right[r_index];
            r_index += 1;
            curr_index += 1;
        }
    }

    pub unsafe fn sort_ptr_arithmatic(arr: *mut i32, left: isize, right: isize) {
        if left >= right {
            return;
        }
        let middle = left + (right - left) / 2;
        sort_ptr_arithmatic(arr, left, middle);
        sort_ptr_arithmatic(arr, middle + 1, right);
        merge_ptr_arithmatic(arr, left as usize, middle as usize, right as usize);
    }

    unsafe fn merge_ptr_arithmatic(arr: *mut i32, l: usize, m: usize, r: usize) {
        let left_amount = m - l + 1;
        let right_amount = r - m;
        let mut left: Vec<i32> = vec![0; left_amount];
        let mut right: Vec<i32> = vec![0; right_amount];
        (0..left_amount).for_each(|i| {
            left[i] = *arr.add(l + i);
        });
        (0..right_amount).for_each(|i| {
            right[i] = *arr.add(m + 1 + i);
        });

        let mut l_index = 0;
        let mut r_index = 0;
        let mut curr_index = l;
        while l_index < left_amount && r_index < right_amount {
            if left[l_index] <= right[r_index] {
                let curr = arr.add(curr_index);
                *curr = left[l_index];
                l_index += 1;
            } else {
                let curr = arr.add(curr_index);
                *curr = right[r_index];
                r_index += 1;
            }
            curr_index += 1;
        }

        let mut s = String::from("");
        for i in 0..4 {
            s.push_str(format!("{}, ", *arr.add(i)).as_str());
        }
        // println!("[{}]", s);

        while l_index < left_amount {
            // println!("left: {}, i: {}", left[l_index], l_index);
            let curr = arr.add(curr_index);
            *curr = left[l_index];
            l_index += 1;
            curr_index += 1;
        }

        while r_index < right_amount {
            // println!("right: {}, i: {}", right[r_index], curr_index);
            let curr = arr.add(curr_index);
            *curr = right[r_index];
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
    fn can_bubble_sort_odd() {
        test_sort_odd_numbers(bubble_sort)
    }

    #[test]
    fn can_merge_sort_odd() {
        test_sort_odd_numbers(merge_sort)
    }

    #[test]
    fn can_bubble_sort_even() {
        test_sort_even_numbers(bubble_sort)
    }

    #[test]
    fn can_merge_sort_even() {
        test_sort_even_numbers(merge_sort)
    }

    fn test_sort_odd_numbers(sort: fn(&mut AutoElementsCritical<i32>)) {
        let mut env = VM.attach_current_thread_as_daemon().unwrap();
        if let Ok(arr) = env.new_int_array(5) {
            unsafe {
                if let Ok(mut arr) = env.get_array_elements_critical(&arr, ReleaseMode::NoCopyBack) {
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

    fn test_sort_even_numbers(sort: fn(&mut AutoElementsCritical<i32>)) {
        let mut env = VM.attach_current_thread_as_daemon().unwrap();
        if let Ok(arr) = env.new_int_array(4) {
            unsafe {
                if let Ok(mut arr) = env.get_array_elements_critical(&arr, ReleaseMode::NoCopyBack) {
                    arr[0] = 5;
                    arr[1] = 2;
                    arr[2] = 10;
                    arr[3] = 0;
                    sort(&mut arr);
                    assert!(arr[0] == 0 && arr[1] == 2 && arr[2] == 5 && arr[3] == 10);
                }
            }
        }
    }
}
