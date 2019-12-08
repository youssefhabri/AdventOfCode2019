fn main() {
    let input = include_str!("../../inputs/day08.txt");
    let width = 25;
    let height = 6;

    let num_layers = input.chars().count() / (width * height);
    let step = width * height;
    let mut layers = vec![];
    for i in (0..input.chars().count()).step_by(step) {
        let layer = input.trim().get(i..(i + step)).unwrap_or("");
        if !layer.is_empty() {
            layers.push(layer)
        }
    }

    let mut min_zeros = (width * height + 1) as i32;
    let mut layer_idx = 0;
    layers.iter().enumerate().for_each(|(idx, layer)| {
        let num_zeros = layer
            .chars()
            .filter_map(|c| if c == '0' { Some(1) } else { None })
            .sum::<i32>();
        if num_zeros < min_zeros {
            min_zeros = num_zeros;
            layer_idx = idx;
        }
    });

    dbg!(&min_zeros);
    dbg!(&layer_idx);

    let num_ones = layers[layer_idx]
        .chars()
        .filter_map(|c| if c == '1' { Some(1) } else { None })
        .sum::<i32>();
    let num_twos = layers[layer_idx]
        .chars()
        .filter_map(|c| if c == '2' { Some(1) } else { None })
        .sum::<i32>();

    println!("{}", num_ones * num_twos);
}
