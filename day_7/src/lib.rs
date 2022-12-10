#[derive(Debug)]
struct Directory<'a> {
    name: &'a str,
    amount: i64,
}

pub fn part_a(input: &str) -> i64 {
    let mut stack = vec![Directory {name: "/", amount: 0}];
    let mut total: i64 = 0;
    for line in input.lines() {

        // We already know that we start at root and we 
        // already know that ls is the primary command
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }
        
        if line.starts_with("$ cd") {
            let name  = line.strip_prefix("$ cd ").unwrap();
            if name == ".." {
                let directory = stack.pop().unwrap();
                if directory.amount <= 100_000 {
                    total += directory.amount;
                }
                stack.last_mut().unwrap().amount += directory.amount;
            } else {
                stack.push(Directory {name, amount: 0})
            }
            continue;
        }

        let (amount, _) = line.split_once(" ").unwrap();
        if let Ok(amount) = amount.parse::<i64>() {
            let mut last = stack.last_mut().unwrap();
            println!("Adding {} to dir {}", amount, last.name);
            last.amount += amount;
        }
    }
    total
}

pub fn part_b(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a_sample() {
        assert_eq!(super::part_a(include_str!("sample.txt")), 95437);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 2104783);
    }

    // #[test]
    // fn part_b_sample() {
    //     assert_eq!(super::part_b(include_str!("sample.txt")), 0);
    // }
    //
    // #[test]
    // fn part_b() {
    //     assert_eq!(super::part_b(include_str!("input.txt")), 0);
    // }
}
