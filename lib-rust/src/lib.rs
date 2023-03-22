#![feature(ptr_metadata)]

pub mod sort;

use jni::{
    objects::{JClass, JIntArray},
    JNIEnv,
};

#[no_mangle]
pub extern "system" fn Java_com_groupawesome_example_rustlib_Sort_bubbleSort<'local>(
    mut env: JNIEnv<'local>,
    _jclass: JClass,
    arr: JIntArray<'local>,
) {
    unsafe {
        if let Ok(mut arr) = env.get_array_elements(&arr, jni::objects::ReleaseMode::CopyBack) {
            sort::bubble_sort(&mut arr);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_groupawesome_example_rustlib_Sort_mergeSort<'local>(
    mut env: JNIEnv<'local>,
    _jclass: JClass,
    arr: JIntArray<'local>,
) {
    unsafe {
        if let Ok(mut arr) = env.get_array_elements(&arr, jni::objects::ReleaseMode::CopyBack) {
            sort::merge_sort(&mut arr);
        }
    }
}
