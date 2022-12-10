const DISK_SPACE: i64 = 70_000_000;
const UNUSED: i64 = 30_000_000;

#[derive(Debug, Copy, Clone)]
struct Directory<'a> {
    name: &'a str,
    amount: i64,
}

pub fn part_a(input: &str) -> i64 {
    let mut stack = vec![Directory {
        name: "/",
        amount: 0,
    }];
    let mut total: i64 = 0;
    for line in input.lines() {
        // We already know that we start at root and we
        // already know that ls is the primary command
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd") {
            let name = line.strip_prefix("$ cd ").unwrap();
            if name == ".." {
                let directory = stack.pop().unwrap();
                if directory.amount <= 100_000 {
                    total += directory.amount;
                }
                stack.last_mut().unwrap().amount += directory.amount;
            } else {
                stack.push(Directory { name, amount: 0 })
            }
            continue;
        }

        let (amount, _) = line.split_once(" ").unwrap();
        if let Ok(amount) = amount.parse::<i64>() {
            let mut last = stack.last_mut().unwrap();
            last.amount += amount;
        }
    }
    total
}

pub fn part_b(input: &str) -> i64 {
    let mut stack = vec![Directory {
        name: "/",
        amount: 0,
    }];
    let mut final_directories = vec![];
    for line in input.lines() {
        // We already know that we start at root and we
        // already know that ls is the primary command
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd") {
            let name = line.strip_prefix("$ cd ").unwrap();
            if name == ".." {
                let directory = stack.pop().unwrap();
                stack.last_mut().unwrap().amount += directory.amount;
                final_directories.push(directory);
            } else {
                stack.push(Directory { name, amount: 0 })
            }
            continue;
        }

        let (amount, _) = line.split_once(" ").unwrap();
        if let Ok(amount) = amount.parse::<i64>() {
            let mut last = stack.last_mut().unwrap();
            last.amount += amount;
        }
    }

    while stack.len() > 0 {
        let directory = stack.pop().unwrap();
        final_directories.push(directory);
        if stack.len() > 0 {
            stack.last_mut().unwrap().amount += directory.amount;
        }
    }

    let free_space = DISK_SPACE - final_directories.last().unwrap().amount;
    let space_required = UNUSED - free_space;
    final_directories
        .into_iter()
        .filter(|d| d.amount >= space_required)
        .map(|d| return d.amount)
        .min()
        .unwrap()
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

    #[test]
    fn part_b_sample() {
        assert_eq!(super::part_b(include_str!("sample.txt")), 24933642);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 5883165);
    }
}
