use std::collections::HashMap;

type Orbits<'a> = HashMap<&'a str, &'a str>;

fn main() {
    let input = include_str!("../../inputs/day06.txt");

    let orbits = input
        .lines()
        .map(|line| {
            let parts = line.split(')').collect::<Vec<_>>();

            (parts[1], parts[0])
        })
        .collect::<Orbits>();

    println!("Part 1: {}", count_orbits(&orbits));
    println!("Part 2: {}", orbits_distance(&orbits, "YOU", "SAN"));
}

fn count_orbits(orbits: &Orbits) -> u32 {
    orbits.keys().fold(0, |acc, mut orbit| {
        let mut count = 0;
        while let Some(new_orbit) = orbits.get(orbit) {
            count += 1;
            orbit = new_orbit
        }

        acc + count
    })
}

fn orbits_distance(orbits: &Orbits, from: &str, to: &str) -> usize {
    let orbit1 = orbits.get(from).unwrap();
    let list1 = orbits_list(&orbits, orbit1);

    let orbit2 = orbits.get(to).unwrap();
    let list2 = orbits_list(&orbits, orbit2);

    for (idx1, orbit1) in list1.iter().enumerate() {
        for (idx2, orbit2) in list2.iter().enumerate() {
            if orbit1 == orbit2 {
                return idx1 + idx2;
            }
        }
    }

    0
}

fn orbits_list<'a>(orbits: &Orbits<'a>, mut orbit: &'a str) -> Vec<&'a str> {
    let mut list = vec![orbit];
    while let Some(new_orbit) = orbits.get(orbit) {
        orbit = new_orbit;
        list.push(new_orbit);
    }

    list
}
