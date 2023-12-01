use std::fs;

fn check_field(field: &str) -> bool {
    let split_field: Vec<&str> = field.split(":").collect();
    match split_field[0] {
        "byr" => {
            let birth_year = split_field[1].parse::<usize>().unwrap();
            if birth_year >= 1920 && birth_year <= 2002 {
                return true;
            } else {
                return false;
            }
        }
        "iyr" => {
            let issue_year = split_field[1].parse::<usize>().unwrap();
            if issue_year >= 2010 && issue_year <= 2020 {
                return true;
            } else {
                return false;
            }
        }
        "eyr" => {
            let expiration_year = split_field[1].parse::<usize>().unwrap();
            if expiration_year >= 2020 && expiration_year <= 2030 {
                return true;
            } else {
                return false;
            }
        }
        "hgt" => {
            let height = &split_field[1];
            if height.contains("cm") {
                let height_number = height.replace("cm", "").parse::<usize>().unwrap();
                if height_number >= 150 && height_number <= 193 {
                    return true;
                } else {
                    return false;
                }
            } else if height.contains("in") {
                let height_number = height.replace("in", "").parse::<usize>().unwrap();
                if height_number >= 59 && height_number <= 176 {
                    return true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        "hcl" => {
            let hair_colour = split_field[1];

            for character in hair_colour.chars().skip(1) {
                // This if statement here is why I'm not uploading this to GitHub
                if character != '0'
                    && character != '1'
                    && character != '2'
                    && character != '3'
                    && character != '4'
                    && character != '5'
                    && character != '6'
                    && character != '7'
                    && character != '8'
                    && character != '9'
                    && character != 'a'
                    && character != 'b'
                    && character != 'c'
                    && character != 'd'
                    && character != 'e'
                    && character != 'f'
                {
                    return false;
                }
            }

            if hair_colour.chars().nth(0).unwrap() == '#' && hair_colour.chars().count() == 7 {
                return true;
            } else {
                return false;
            }
        }
        "ecl" => {
            let eye_colour = split_field[1];
            if eye_colour == "amb"
                || eye_colour == "blu"
                || eye_colour == "brn"
                || eye_colour == "gry"
                || eye_colour == "grn"
                || eye_colour == "hzl"
                || eye_colour == "oth"
            {
                return true;
            } else {
                return false;
            }
        }
        "pid" => {
            let passport_id = split_field[1];
            for character in passport_id.chars() {
                if character.is_numeric() == false {
                    return false;
                }
            }
            // let passport_id_numbers = split_field[1].parse::<usize>().unwrap();

            if passport_id.chars().count() == 9 {
                return true;
            } else {
                return false;
            }
        }
        "cid" => true,
        _ => {
            println!(
                "WARNING! Invalid field {} {}",
                split_field[0], split_field[1]
            );
            return false;
        }
    }
}

fn part_one_check(passport: &Vec<&str>) -> bool {
    let count: usize;
    count = passport.iter().filter(|field| &field[..3] != "cid").count();
    if count == 7 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Unable to read file");

    let mut passports: Vec<Vec<&str>> = Vec::new();
    let mut passport_temp: Vec<&str> = Vec::new();
    for line in input.lines() {
        if line.chars().count() == 0 {
            // Appends the temp value to the full vector.
            // We do this when the count is zero because an empty line is used to separate values
            //
            //
            //
            //
            //
            //
            //
            //
            //
            //
            // WARNING!
            // Using this method means that the last line is never counted, meaning that the part one answer can be off by one.
            //
            //
            //
            //
            //
            //
            //
            //
            //
            //
            passports.push(passport_temp.clone());
            passport_temp.clear();
        } else {
            // Splits the current line by whitespace and appends it to the passport_temp variable
            passport_temp.append(&mut line.split_whitespace().collect::<Vec<&str>>());
        }
    }

    // Part 1
    let mut valid_passports = 0;
    for passport in &passports {
        if part_one_check(passport) == true {
            valid_passports += 1;
        }
    }

    println!("Part one answer: {}", valid_passports);

    valid_passports = 0;
    for passport in &passports {
        if part_one_check(passport) == true {
            let mut has_failed = false;
            for field in passport {
                if check_field(field) == false {
                    has_failed = true;
                }
            }
            if has_failed == false {
                valid_passports += 1;
            }
        }
    }

    println!("Part two answer: {}", valid_passports);
}
