pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song = String::new();
    let numbers = [
        "No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
    ];

    for n in ((start_bottles - take_down + 1)..=start_bottles).rev() {
        let slice = format!(
            "{x} green bottle{plural_x} hanging on the wall,\n{x} green bottle{plural_x} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {y} green bottle{plural_y} hanging on the wall.\n\n",
            x = numbers[n as usize],
            y = numbers[(n - 1) as usize].to_ascii_lowercase(),
            plural_x = if numbers[n as usize].eq_ignore_ascii_case("one") {
                ""
            } else {
                "s"
            },
            plural_y = if numbers[(n - 1) as usize].eq_ignore_ascii_case("one") {
                ""
            } else {
                "s"
            },
        );
        song.push_str(&slice);
    }

    song.trim().to_string()
}
