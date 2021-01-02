use std::fs;

pub fn work_on_file<F, T>(path: &str, fun: F) -> Vec<T>
where
    F: Fn(&str) -> T,
{
    let lines = fs::read_to_string(path).expect("Could not read file");
    lines.lines().map(fun).collect()
}

pub fn read_all_lines<F, T>(path: &str) -> Vec<String> {
    let lines = fs::read_to_string(path).expect("Could not read file");
    lines.lines().map(String::from).collect()
}

pub fn read_single_line(path: &str) -> String {
    let lines = fs::read_to_string(path).expect("Could not read file");
    match lines.lines().next() {
        Some(line) => String::from(line.trim()),
        None => String::from(""),
    }
}

pub fn as_int(input: &str) -> i32 {
    input.parse().unwrap()
}

/*
pub fn read_file2<'a, F, T>(path: &str, fun: &'a F) -> MyIterator<'a, F, T>
where
F: Fn(&str) -> T,
{
    let input = fs::read_to_string(path).expect("Could not read file");
    let lines = input.lines().clone();
    MyIterator { lines, fun }
}

struct MyIterator<'a, F, T>
where
F: Fn(&str) -> T,
{
    fun: &'a F,
    lines: Lines<'a>,
}

impl<'a, F, T> Iterator for MyIterator<'a, F, T>
where
F: Fn(&str) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next()?;
        Some((self.fun)(line))
    }
}
*/
