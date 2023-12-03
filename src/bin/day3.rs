use itertools::Itertools;

#[derive(Debug)]
struct PartNumber {
    number: u32,
    len: u32,
    line: usize,
    column: usize,
}

fn main() {
    let input = std::fs::read_to_string("inputs/day3/input.txt").expect("Unable to read file");

    let lines = input.lines().collect_vec();

    let parts = lines
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(idx, line)| {
            let mut result = Vec::new();
            let mut offset = 0;
            while let Some(start) = line[offset..].find(|c: char| c.is_ascii_digit()) {
                let end = line[offset + start..]
                    .find(|c: char| !c.is_ascii_digit())
                    .map(|i| i + start)
                    .unwrap_or_else(|| line.len() - offset);
                let number = line[offset + start..offset + end].parse().unwrap();
                result.push(PartNumber {
                    number,
                    len: (end - start) as u32,
                    line: idx,
                    column: offset + start,
                });
                offset += end;
            }
            result.into_iter()
        })
        .collect_vec();

    // Check around each part if there is a symbol
    let part_numbers = parts
        .iter()
        .filter(|part| {
            for line in lines
                .iter()
                .take((part.line + 1).min(lines.len() - 1) + 1)
                .skip(part.line.saturating_sub(1))
            {
                for x in (part.column.saturating_sub(1))..=(part.column + part.len as usize) {
                    let c = line.chars().nth(x);
                    if matches!(c, Some(c) if !(c.is_ascii_digit() || c == '.')) {
                        return true;
                    }
                }
            }
            false
        })
        .collect_vec();

    let solution1 = part_numbers.iter().map(|part| part.number).sum::<u32>();
    println!("Solution 1: {solution1}");

    let mut solution2 = 0;
    for (y, line) in lines.iter().copied().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '*' {
                continue;
            }
            // Find two parts
            let adjacent_parts = parts.iter().filter(|part| {
                let px = part.column;
                let py = part.line;
                let len = part.len;
                (px.saturating_sub(1)..=(px + len as usize)).contains(&x)
                    && (py.saturating_sub(1)..=(py + 1)).contains(&y)
            });

            let Some((first, second)) = adjacent_parts.collect_tuple() else {
                continue;
            };

            let ratio = first.number * second.number;
            solution2 += ratio;
        }
    }

    println!("Solution 2: {solution2}");
}
