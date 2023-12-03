use std::{fs, collections::HashMap};

#[derive(Debug)]
struct NumLocation {
    num: u32,
    location: u32,
}

fn main() {
    let codes = fs::read_to_string("codes.txt").unwrap();
    let mut calibration_values = Vec::new();

    for code in codes.lines() {
        let mut combined_locations: Vec<NumLocation> = Vec::new();
        let mut num_string_locations = find_num_strings(code);
        let mut num_char_locations = find_num_chars(code);
        
        combined_locations.append(&mut num_char_locations);
        combined_locations.append(&mut num_string_locations);
        combined_locations.sort_by(|a, b| a.location.cmp(&b.location));

        calibration_values.push(combined_locations[0].num * 10 + combined_locations[combined_locations.len() - 1].num);
    }

    let sum: u32 = calibration_values.iter().sum();

    print!("{:?}", calibration_values);

    println!("Sum is: {}", sum);
}

fn find_num_strings(s: &str) -> Vec<NumLocation> {
    let num_map: HashMap<&str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ].iter().copied().collect();
    let keys = num_map.keys();

    let mut num_locations: Vec<NumLocation> = Vec::new(); 

    for key in keys.into_iter() {
        let first_match = s.find(key);
        let last_match = s.rfind(key);
        
        match first_match {
            Some(found_value) => num_locations.push(NumLocation { num: num_map[key], location: found_value as u32 }),
            None => ()
        }
        match last_match {
            Some(found_value) => num_locations.push(NumLocation { num: num_map[key], location: found_value as u32 }),
            None => ()
        }
    }

    num_locations
}

fn find_num_chars(s: &str) -> Vec<NumLocation> {
     let first_num_position = s.chars().position(|c| c.is_numeric());
     let last_num_position_from_end = s.chars().rev().position(|c| c.is_numeric());
     let mut num_locations: Vec<NumLocation> = Vec::new();

     match first_num_position {
        Some(position) => {
            let first_num_value = s.chars().nth(position).unwrap();
            num_locations.push(NumLocation { num: first_num_value.to_digit(10).unwrap(), location: position as u32 });
        },
        None => (),
     }

     match last_num_position_from_end {
        Some(position) => {
            let last_num_position = s.chars().count() - 1 - position;
            let last_num_value = s.chars().nth(last_num_position).unwrap();
            num_locations.push(NumLocation { num: last_num_value.to_digit(10).unwrap(), location: last_num_position as u32 });
        },
        None => (),
     }

     num_locations
}
