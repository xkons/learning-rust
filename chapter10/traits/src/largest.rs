fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn welches_wort_ist_groesser<'a>(wort1: &'a str, wort2: &'a str) -> &'a str {
    if wort1 > wort2 { return wort1 } else { return wort2 }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // For experimenting with sizes of german words:
    println!("Größeres Wort: {}", welches_wort_ist_groesser("zzzzz", "zaunö"));
}