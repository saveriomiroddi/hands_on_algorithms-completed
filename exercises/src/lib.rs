// Required to avoid warnings; the compiler doesn't recognize the invocation passed to the testing
// macro.
//
#![allow(unused_imports)]
// For convenience. Some methods aren't worth testing, because they're a copy of the course.
//
#![allow(dead_code)]

mod helpers;

// Don't forget to add all the modules, so that the UTs are run.
//
pub mod d2_1_bubble_sort;
pub mod d2_2_merge_sort;
pub mod d2_2_merge_sort_source;
pub mod d2_3_quicksort;
pub mod d2_4_dynamic_programming;
pub mod d3_1_linked_list;
