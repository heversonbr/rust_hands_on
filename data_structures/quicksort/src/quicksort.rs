

fn partition(sorting_arr: &mut [i32], low: usize, high: usize) -> usize {
//
    let mut i = low;
    let mut j = high;
    let pivot_idx = low;
//
    println!("PARTITIONING ARRAY: {:?}" , &sorting_arr[low..high+1]);
    println!("Pivot Index: {:?}   /  i: {:?} , j: {:?}" , pivot_idx , i , j);
    while i < j{
        while sorting_arr[i] <= sorting_arr[pivot_idx] && i < high{
            i+=1;
        }
        println!("i: {:?}" , i);
        while sorting_arr[j] > sorting_arr[pivot_idx] && j > low{
            j-=1; 
        }
        println!("j: {:?}" , j);
        if i<j{
            println!("[i<j] -> swap: {:?} and {:?}" , sorting_arr[i], sorting_arr[j]);
            (sorting_arr[i], sorting_arr[j]) = (sorting_arr[j],sorting_arr[i]);
        }
    }

    println!("Swapping pivot index: {:?} with j_index: {:?}" , pivot_idx, j);
    println!("Swapping pivot value: {:?} with j_value: {:?}" , sorting_arr[pivot_idx], sorting_arr[j] );
    (sorting_arr[pivot_idx] , sorting_arr[j]) = (sorting_arr[j] , sorting_arr[pivot_idx]);
    println!("After swap: {:?}" , sorting_arr);
    j

}

pub fn quicksort(sorting_arr: &mut [i32], low: usize, mut high: usize){

    println!("QUICKSORT using low: {:?} and high: {:?}", low, high);
    if low < high {
        high-=1;
        
  
        // 1) find a pivot, such that all the values to the left are lower than the pivot, and all the values to the right are higher than the pivot
        let partition_idx: usize = partition(sorting_arr, low, high);
        println!("partition_idx: {:?} ", partition_idx);
        // 2) Apply the quicksort algorithm recursively to each partition (low->pivot-1) and (pivot+1->high)
        if partition_idx > 0 {
            quicksort(sorting_arr, low, partition_idx-1);
        }
        
        quicksort(sorting_arr, partition_idx+1, high);
        
    }


}