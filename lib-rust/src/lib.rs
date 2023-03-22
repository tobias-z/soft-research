#![feature(ptr_metadata)]

pub mod sort;

use std::ptr::addr_of_mut;

use jni::{
    objects::{JClass, JIntArray},
    sys::jboolean,
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

#[no_mangle]
pub extern "system" fn Java_com_groupawesome_example_rustlib_Sort_mergeSortWithArithmatic<
    'local,
>(
    env: JNIEnv<'local>,
    _jclass: JClass,
    arr: JIntArray<'local>,
) {
    unsafe {
        let env = env.get_native_interface();
        let mut bool: jboolean = 1;
        let elements = (**env).GetIntArrayElements.unwrap()(env, arr.as_raw(), addr_of_mut!(bool));
        let length = (**env).GetArrayLength.unwrap()(env, arr.as_raw());
        sort::merge_sort_ptr_arithmatic(elements, length);
        (**env).SetIntArrayRegion.unwrap()(env, arr.as_raw(), 0, 2, elements);
    }
}
