#![feature(ptr_metadata)]

pub mod sort;

use std::ptr::addr_of_mut;

use jni::{
    objects::{JClass, JIntArray},
    sys::jboolean,
    JNIEnv,
};

#[no_mangle]
pub extern "system" fn Java_com_example_ufoproject_rust_RustSort_bubbleSort<'local>(
    mut env: JNIEnv<'local>,
    _jclass: JClass,
    arr: JIntArray<'local>,
) {
    unsafe {
        if let Ok(mut arr) = env.get_array_elements_critical(&arr, jni::objects::ReleaseMode::CopyBack) {
            sort::bubble_sort(&mut arr);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_example_ufoproject_rust_RustSort_mergeSort<'local>(
    mut env: JNIEnv<'local>,
    _jclass: JClass,
    arr: JIntArray<'local>,
) {
    unsafe {
        if let Ok(mut arr) = env.get_array_elements_critical(&arr, jni::objects::ReleaseMode::CopyBack) {
            sort::merge_sort(&mut arr);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_example_ufoproject_rust_RustSort_sortWithArithmetic<
    'local,
>(
    env: JNIEnv<'local>,
    _jclass: JClass,
    arr: JIntArray<'local>,
) {
    unsafe {
        let env = env.get_native_interface();
        let mut bool: jboolean = 0;
        let arr_ptr = arr.as_raw();
        let elements = (**env).GetIntArrayElements.unwrap()(env, arr_ptr, addr_of_mut!(bool));
        let length = (**env).GetArrayLength.unwrap()(env, arr_ptr);
        sort::merge_sort_ptr_arithmatic(elements, length);
        (**env).ReleaseIntArrayElements.unwrap()(env, arr_ptr, elements, 0)
    }
}
