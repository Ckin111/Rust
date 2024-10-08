// Vectors are just resizable arrays

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    // reassign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(6);

    // pop off last value 
    numbers.pop();

    println!("{:?}",numbers);

    // Get single val 
    println!("{}",numbers[0]);

    // Get vector length
    println!("Vector Length: {}", numbers.len());

    // Vector are stack allocated
    println!("Vector occupies: {} bytes", std::mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}",x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *=2;
    }
    println!("Numbers Vec: {:?}", numbers);
}