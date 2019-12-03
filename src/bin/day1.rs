fn main() {
    let input = include_str!("../../inputs/day01.txt");

    let sum: i32 = input
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .map(compute_fuel)
        .sum();

    println!("Fuel sum: {}", sum);
}

fn compute_fuel(mass: i32) -> i32 {
    let fuel = (mass / 3) - 2;
    if fuel <= 0 {
        return 0;
    }

    fuel + compute_fuel(fuel)
}
