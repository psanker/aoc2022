mod day1;

fn main() {
    let d1p1 = day1::day1pt1("inputs/day1.txt").ok().unwrap();
    println!("Day 1 pt 1: {}", d1p1);

    let d1p2 = day1::day1pt2("inputs/day1.txt").ok().unwrap();
    println!("Day 1 pt 2: {}", d1p2);
}
