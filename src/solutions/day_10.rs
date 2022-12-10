pub fn run() {
    let input_str = include_str!("../../inputs/input_10.txt");

    let input: Vec<(&str, i32)> = input_str
        .lines()
        .map(|line| {
            let line_split: Vec<&str> = line
            .split_ascii_whitespace()
            .collect();

            (line_split[0], line_split.get(1).unwrap_or(&"0").parse().unwrap())
        })
        .collect();

    part_one(&input);
}

fn part_one(input: &Vec<(&str, i32)>) {
    let mut signal_strength_sum = 0;
    let mut x_register = 1;
    let mut cycle = 0;

    for &(command, arg) in input {
        cycle += 1;
        if cycle % 40 == 20 {
            signal_strength_sum += cycle * x_register;
        }

        match command {
            "noop" => (),
            "addx" => {
                cycle += 1;
                if cycle % 40 == 20 {
                    signal_strength_sum += cycle * x_register;
                }
                x_register += arg;
            },
            _ => println!("Invalid command '{command}'")
        };
    }

    println!("Part one: {signal_strength_sum}");
}
