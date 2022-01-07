// Reference Pointers - Points to a resource in memory

pub fn run() {
    // Primitive Array - Allocated on Stack, because it is fixed size
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = arr1;

    println!("Array Values: {:?}", (arr1, arr2));

    // Vector - Allocated on Heap, because it is resizable
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = &vec1; // Reference to vec1, because vec1 is on Heap

    println!("Vector Values: {:?}", (&vec1, vec2));
}
