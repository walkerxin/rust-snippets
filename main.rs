//固定i32类型排序
fn bubble_sort_i32(arr: &mut [i32]) { let mut swapped = true; while swapped { swapped = false; for i in 1..arr.len() { if arr[i - 1] > arr[i] { arr.swap(i - 1, i); swapped = true; } } }
}

//泛型排序
fn bubble_sort<T: PartialOrd>(arr: &mut [T]) { let mut swapped = true; while swapped { swapped = false; for i in 1..arr.len() { if arr[i - 1] > arr[i] { arr.swap(i - 1, i); swapped = true; } } }
}

fn main() { let mut arr1 = [5, 2, 6, 3, 1]; bubble_sort_i32(&mut arr1); let mut arr2 = ["c", "b", "a", "e", "d"]; bubble_sort(&mut arr2); println!("{:?}", arr1); println!("{:?}", arr2);
}
