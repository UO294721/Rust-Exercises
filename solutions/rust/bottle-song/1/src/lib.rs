pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut i = 0;
    let mut poem = String::new();
    let second_string = String::from("And if one green bottle should accidentally fall,\n");
    let third_string_part_one = String::from("There'll be ");
    let bottle_string = String::from(" green bottle");
    let hang_string = String::from(" hanging on the wall");
    let cont = String::from(".\n\n");
    let end = String::from(".");
    let verse = String::from(",\n");
    while i < take_down {
        poem.push_str(&(capitalize_first(num2words(start_bottles - i)).to_owned() + &bottle_string + &various_bottles(start_bottles - i) + &hang_string + &verse));
        poem.push_str(&(capitalize_first(num2words(start_bottles - i)).to_owned() + &bottle_string + &various_bottles(start_bottles - i) + &hang_string + &verse));
        poem.push_str(&second_string);
        i += 1;
        poem.push_str(&(third_string_part_one.clone() + &num2words(start_bottles - i).to_owned() + &bottle_string +  &various_bottles(start_bottles - i) + &hang_string));

        if i < take_down {
            poem.push_str(&cont);
        } else {
            poem.push_str(&end);
        }
        
    };
    poem
}

fn various_bottles(num: u32) -> &'static str {
    if num > 1 || num == 0 {
        "s"
    } else {
        ""
    }
}

fn num2words(num: u32) -> &'static str {
        match num {
            0 => "no",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            _ => "none"
        }
}

        
fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}