use std::io;

fn run(lecture_hall_coffees: Vec<bool>) -> u32 {
    let mut coffee_count = 0u8;
    let mut lectures_awake = 0u32;

    for lecture_hall in lecture_hall_coffees {
        if lecture_hall {
            lectures_awake += 1;
            coffee_count = 2;
        } else if coffee_count > 0 {
            lectures_awake += 1;
            coffee_count -= 1;
        }
    }

    lectures_awake
}

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|line_res| line_res.unwrap())
        .collect::<Vec<_>>();

    let lecture_hall_coffees = lines[1]
        .chars()
        .map(|coffee_int| match coffee_int {
            '1' => true,
            '0' => false,
            _ => unreachable!(),
        })
        .collect();

    let res = run(lecture_hall_coffees);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let lecture_hall_coffees = vec![
            false, true, false, false, false, true, false, true, false, false,
        ];

        assert_eq!(run(lecture_hall_coffees), 8);
    }

    #[test]
    fn test_example_2() {
        let lecture_hall_coffees = vec![
            true, true, false, false, false, false, false, false, false, false,
        ];

        assert_eq!(run(lecture_hall_coffees), 4);
    }

    #[test]
    fn test_example_3() {
        let lecture_hall_coffees = vec![false];

        assert_eq!(run(lecture_hall_coffees), 0);
    }
}
