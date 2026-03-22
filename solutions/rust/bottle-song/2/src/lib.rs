pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let num_to_string = [
        "No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
    ];

    let mut song = String::new();
    for i in 0..take_down {
        let bottle_from = (start_bottles - i) as usize;
        let first_line = format!(
            "{} green {} hanging on the wall,\n",
            num_to_string[bottle_from],
            bottle_for_num(bottle_from)
        );
        let second_line = first_line.clone();
        let third_line = format!(
            "There'll be {} green {} hanging on the wall.\n",
            num_to_string[bottle_from - 1].to_lowercase(),
            bottle_for_num(bottle_from - 1)
        );

        song.push_str(&first_line);
        song.push_str(&second_line);
        song.push_str("And if one green bottle should accidentally fall,\n");
        song.push_str(&third_line);
        song.push('\n');
    }
    song
}

fn bottle_for_num(n: usize) -> String {
    match n {
        1 => "bottle",
        _ => "bottles",
    }
    .to_string()
}
