fn basic_impl() {
    println!("This is a grep tool");
    let base_phrase: String = "This is a string against which we will use the grep tool where we have a small search string phrase".to_string();


    let mut search_phrase: String = String::new();
    println!("Enter search phrase");
    std::io::stdin().read_line(&mut search_phrase).expect("Failed to read the search phrase");

    // For now let us have a string against which we will search
    // Method 1: Use contains()
    if base_phrase.contains(&search_phrase.trim()) {    // need to use .trim() to clean the end \n character
        println!("Method1: Found the substring");
    }

    // Method 2: Use find() -> gives the index too
    let index: Option<usize> = base_phrase.find(&search_phrase.trim());
    match index {
        Some(index) => println!("Method2: Found the substring at index {:?}", index),
        None => println!("Method2: Did not find the string"),
    }


}
