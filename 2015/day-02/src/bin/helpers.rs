pub fn string_to_nums(input_string: &String) -> Result<[u32; 3], &'static str> {
    let pieces: Vec<u32> = input_string
        .split("x")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    if pieces.len() != 3 {
        Err("Incorrect input format!")
    } else {
        Ok([pieces[0], pieces[1], pieces[2]])
    }
}
