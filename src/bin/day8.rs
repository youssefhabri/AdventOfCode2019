fn main() {
    let input = include_str!("../../inputs/day08.txt");
    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;

    let num_layers = input.chars().count() / (WIDTH * HEIGHT);
    let step = WIDTH * HEIGHT;
    let mut layers = vec![];
    for i in (0..input.chars().count()).step_by(step) {
        let layer = input.trim().get(i..(i + step)).unwrap_or("");
        if !layer.is_empty() {
            layers.push(layer)
        }
    }

    // Generate the message
    let mut new_image = ['2'; WIDTH * HEIGHT];
    layers.iter().for_each(|layer| {
        layer.chars().enumerate().for_each(|(i, c)| {
            if new_image[i] == '2' {
                new_image[i] = c;
            }
        })
    });

    // Print the Message
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", new_image[x + (y * WIDTH)]);
        }
        println!();
    }
}
