// IKinder

pub fn run() {
    let colors = vec!["red".to_owned(), "green".to_owned(), "blue".to_owned()];
    print_elements(&colors);

    let mut long_colors = colors.clone();
    shorten_strings(&mut long_colors);
    print_elements(&long_colors);

    let upper_colors = to_uppercase(&colors);
    print_elements(&upper_colors);

    let mut moved_colors = vec![];
    move_elements(upper_colors, &mut moved_colors);
    print_elements(&moved_colors);

    let exploded_colors = explode(&moved_colors)
        .into_iter()
        .flatten()
        .collect::<Vec<String>>();
    print_elements(&exploded_colors);

    let found_color = find_color_or(&colors, "re", "orange");
    println!("{}", found_color);
}

fn print_elements(elements: &[String]) {
    elements.iter().for_each(|element| print!("{} ", element));
    println!();
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|element| element.truncate(1))
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|element| element.to_uppercase())
        .collect()
}

fn move_elements(from: Vec<String>, into: &mut Vec<String>) {
    from.into_iter().for_each(|element| into.push(element))
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|element| element.chars().map(|char| char.to_string()).collect())
        .collect()
}

fn find_color_or<'a>(elements: &'a [String], search: &str, fallback: &'a str) -> &'a str {
    // elements
    // 	.iter()
    // 	.map(|element| element.as_str())
    // 	.find(|&element| element.contains(search))
    // 	.unwrap_or(fallback)
    elements
        .iter()
        .find(|&element| element.contains(search))
        .map_or(fallback, |result| result.as_str())
}
