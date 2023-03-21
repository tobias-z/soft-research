use jni::objects::AutoElements;

// TODO: Ensure that we are doing bubble sort the most optimally
pub fn bubble_sort(arr: &mut AutoElements<i32>) {
    for _i in 0..arr.len() {
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

fn merge_sort(arr: &mut AutoElements<i32>) {
    todo!()
}

#[cfg(test)]
mod test {
    use jni::{InitArgsBuilder, JNIVersion, JavaVM};
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
        test_sort(|arr| {
            bubble_sort(arr);
        })
    }

    #[test]
    fn can_merge_sort() {
        test_sort(|arr| {
            merge_sort(arr);
        })
    }

    fn test_sort(sort: fn(&mut AutoElements<i32>)) {
        let mut env = VM.attach_current_thread().unwrap();
        if let Ok(arr) = env.new_int_array(5) {
            unsafe {
                if let Ok(mut arr) =
                    env.get_array_elements(&arr, jni::objects::ReleaseMode::CopyBack)
                {
                    arr[0] = 5;
                    arr[1] = 2;
                    arr[2] = 10;
                    arr[3] = 0;
                    arr[4] = -2;
                    sort(&mut arr);
                    assert!(
                        arr[0] == -2 &&
                        arr[1] == 0 &&
                        arr[2] == 2 &&
                        arr[3] == 5 &&
                        arr[4] == 10
                    );
                }
            }
        }
    }
}
