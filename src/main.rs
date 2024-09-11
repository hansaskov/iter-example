fn main() {
    let values = [
        0.9, 0.8, 1.2, 1.5, 10.3, 0.7, 2.5, 4.1, 0.7, 2.5, 4.1, 0.2, -3.0, 0.6, 1.3, 2.6, 0.9,
    ];
    println!("Runs");
    let range: Vec<_> = values
        .iter()
        .enumerate()
        .filter(|(_i, &x)| x > 1.0)
        .map(|(i, _x)| i)
        .collect::<Vec<usize>>()
        .chunk_by(|lhs, rhs| rhs - lhs < 3)
        .filter_map(|slice| {
            let beginning = *slice.first()?;
            let end = *slice.last()? + 1;
            return Some((beginning, end));
        })
        .collect();

    for (b, e) in &range {
        println!("{b:>2} {e:>2}");
    }

    println!("Longest run");
    let max = range.iter().max_by_key(|(front, back)| back - front);
    match max {
        Some((b, e)) => println!("{b:>2} {e:>2}"),
        None => println!("Empty!"),
    }
}
