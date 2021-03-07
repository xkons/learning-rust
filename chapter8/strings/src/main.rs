fn main() {
    // creating an empty string
    let s1 = String::new();
    println!("empty string: {:?}", s1);

    // creating a string with initial data
    let data = "initial contents";
    let s2 = data.to_string();
    println!("{}", s2);

    // creating a string with initial data by using the associated String::from method
    let s3 = String::from("initial contents with String::from");
    println!("{}", s3);

    // concatenating with the + operator
    let sc1 = String::from("tic");
    let sc2 = String::from("tac");
    let sc3 = String::from("toe");

    let sc4 = sc1 + "-" + &sc2 + "-" + &sc3; // sc1 is no longer usable as sc4 takes ownership
    println!("concatenated with +: {}", sc4);
    // println!("sc1: {}", sc1); <- not possible anymore


    // concatenating strings using format!, which doesn't take ownership
    let sc5 = String::from("tic");
    let sc6 = String::from("tac");
    let sc7 = String::from("toe");

    let sc8 = format!("{}-{}-{}", sc5, sc6, sc7);
    println!("concatenated with format: {}", sc8);

    let hindi = String::from("नमस्ते");
    let chars = hindi.chars();
    println!("{:?}", chars);
}
