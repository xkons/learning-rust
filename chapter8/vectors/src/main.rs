fn main() {
    let mut v1 = Vec::new();
    // pushing elements into a vector
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);
    println!("v1: {:?}", v1);

    // create a vector with initial values using vec!
    let v2 = vec![1, 2, 3];
    println!("v2: {:?}", v2);

    // accessing values in a vector
    let v3 = vec![1, 2, 3, 4, 5];

    // using indexing syntax
    let third: &i32 = &v3[2];
    println!("The third element is {}", third);

    // using Vec.get
    match v3.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // out of bounds
    // let v = vec![1, 2, 3, 4, 5];
    //let does_not_exist = &v[100]; // this will cause the program to panic

    // iterating over elements in immutable vector
    let v4 = vec![100, 32, 57];
    for i in &v4 {
        println!("iterating over v4: {}", i);
    }

    // iteratoring over elements in mutable vector to make changes to each element
    let mut v5 = vec![100, 32, 57];
    println!("original v5: {:?}", v5);
    for i in &mut v5 {
        *i += 50; // more info on dererference operator: https://doc.rust-lang.org/book/ch15-02-deref.html#following-the-pointer-to-the-value-with-the-dereference-operator
    }
    println!("mutated v5 with every element +50: {:?}", v5);


    // using enums to hold different types in the same vector
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}
