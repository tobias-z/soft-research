#![feature(ptr_metadata)]

pub mod binary_search;
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
        if let Ok(mut arr) =
            env.get_array_elements_critical(&arr, jni::objects::ReleaseMode::CopyBack)
        {
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
        if let Ok(mut arr) =
            env.get_array_elements_critical(&arr, jni::objects::ReleaseMode::CopyBack)
        {
            sort::merge_sort(&mut arr);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_example_ufoproject_rust_RustBinarySearchTree_binarySearch<
    'local,
>(
    mut env: JNIEnv<'local>,
    _jclass: JClass,
    sorted_arr: JIntArray<'local>,
    numbers_to_find: JIntArray<'local>,
) -> JIntArray<'local> {
    unsafe {
        let sorted_arr = env.get_array_elements(&sorted_arr, jni::objects::ReleaseMode::NoCopyBack);
        let numbers_to_find =
            env.get_array_elements(&numbers_to_find, jni::objects::ReleaseMode::NoCopyBack);
        match (sorted_arr, numbers_to_find) {
            (Ok(sorted_arr), Ok(numbers_to_find)) => {
                let found_indexes: Vec<i32> =
                    binary_search::search_multiple(sorted_arr, numbers_to_find);
                let arr = env.new_int_array(found_indexes.len() as i32).unwrap();
                env.set_int_array_region(&arr, 0, found_indexes.as_slice())
                    .expect("unable to put the found indexes into the returned int array");
                arr
            }
            _ => env.new_int_array(0).unwrap(),
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_example_ufoproject_rust_RustSort_sortWithArithmetic<'local>(
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
