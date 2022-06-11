/// Author: XQtbl

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        assert_eq!(
            super::solve(1, 2),
            3
        );
    }
}

fn get_a_b() -> (u8, u8) {
    use std::io::stdin;
    use std::io::Read;
    let cin = stdin();
    let mut cin = cin.lock();
    let mut ret = [0; 3];
    cin.read(&mut ret);
    (ret[0] - b'0', ret[2] - b'0')
}

fn solve(a: u8, b: u8) -> u8 {
    a + b
}

pub fn main() {
    let (a, b) = get_a_b();
    println!("{}", solve(a, b));
}