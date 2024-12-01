fn sum(reader: impl std::io::BufRead) -> i32 {
    let mut lefts = Vec::new();
    let mut rights = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if !line.is_empty() {
            let mut parts = line.split_whitespace();
            let left = parts.next().unwrap();
            let right = parts.next().unwrap();
            lefts.push(left.trim().parse::<i32>().unwrap());
            rights.push(right.trim().parse::<i32>().unwrap());
        }
    }
    lefts.sort_unstable();
    rights.sort_unstable();
    lefts
        .into_iter()
        .zip(rights)
        .map(|(left, right)| (left - right).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            11,
            sum(
                "3   4
4   3
2   5
1   3
3   9
3   3"
                    .as_bytes()
            )
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
