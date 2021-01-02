use common::read_single_line;
use computer::Computer;

pub fn result1() {
    let input = read_single_line("data/day02/input.txt");
    let mut computer = Computer::parse(&input);
    computer.patch_memory(1, 12);
    computer.patch_memory(2, 2);

    computer.run();
    let result = computer.memory()[0];

    println!("Day 02 - Result 1: {}", result);
}

pub fn result2() {
    let input = read_single_line("data/day02/input.txt");
    let computer = Computer::parse(&input);

    for noun in 0..100 {
        for verb in 0..100 {
            let mut computer = computer.clone();
            computer.patch_memory(1, noun);
            computer.patch_memory(2, verb);

            let computer = computer.run();
            let result = computer.memory()[0];
            if result == 19690720 {
                println!("Day 02 - Result 2: {}", 100 * noun + verb);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use computer::Computer;

    #[test]
    fn test_parse() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        let computer = Computer::parse(&input);
        let result = computer.memory();
        let expected = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(result, &expected);
    }

    #[test]
    fn test_computer_running() {
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        let mut computer = Computer::new(input);
        computer.run();
        let result = computer.memory();

        let expected: Vec<i32> = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        assert_eq!(result, &expected);
    }
}
