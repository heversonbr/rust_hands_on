#[allow(unused_doc_comments)]
#[allow(dead_code)]
pub fn regular_selection_sort(sorting_arr: &mut [i32]) {
    /// This function receives a reference for a mutable array
    /// it applies the 'selection sort' algorith to the array 
    /// 

    /// it is called regular sort just because I dont use recursion in this case.
    // note that: the selection algorithm is not very efficient because it 
    println!("Array length: {:?}", sorting_arr.len());
    let size = sorting_arr.len();
    // note: len returns 'usize' type
    if size <= 0 { return }
    
    for x in 0..(size-1) {
        let mut min_index = x; 
        let start = x;

        for i in (start+1)..(size) {
            if sorting_arr[i] < sorting_arr[min_index] { 
                min_index = i;
            }
        }
        if min_index != start{  
            (sorting_arr[x], sorting_arr[min_index]) = (sorting_arr[min_index], sorting_arr[x]);
        }
    }
}

#[allow(dead_code)]
#[allow(unused_doc_comments)]
pub fn recursion_selection_sort(sorting_arr: &mut [i32], start: usize, size: usize) {

    let min_index = start;

    for i in (start+1)..(size) {
        if sorting_arr[i] < sorting_arr[min_index] { 
            (sorting_arr[i], sorting_arr[min_index]) = (sorting_arr[min_index], sorting_arr[i]);
        }
    }
    if start < size {  
        recursion_selection_sort(sorting_arr, start+1, size);
    }



}