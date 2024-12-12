pub fn hanoi(n: u32, from: u32, to: u32, via: u32) {
    if n == 1 {
        println!("Move disk from {} to {}", from, to);
    } else {
        hanoi(n - 1, from, via, to);
        hanoi(1, from, to, via);
        hanoi(n - 1, via, to, from);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hanoi() {
        hanoi(3, 1, 3, 2);
    }
}