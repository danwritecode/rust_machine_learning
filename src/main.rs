use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    // y = x * w; solve for w
    let mut w = rng.gen::<f32>() * 10.0;

    let eps = 1e-3; // smaller the epsilon, the more accurate
    let rate = 1e-3; // create a rate of change (going to apply to diff lossiness)

    for _i in 0..500 {
        let der_loss = (lossiness(w + eps) - lossiness(w)) / eps; // use derivative to find
        w -= rate * der_loss; // mutating w each time
        dbg!(lossiness(w));
    }

    dbg!(w);
}

/// Lossiness is used to calculate on average, how far off w is from control
fn lossiness(w: f32) -> f32 {
    let training_data = get_training_data();
    let mut result = 0.0;

    for data in &training_data {
        let x = data.0;
        let y = x as f32 * w; // y = x * w;
        let diff = y - data.1 as f32; // calculate how far off from control
        result += diff * diff; // used to amplify for ease of reading/reasoning
    }

    result /= training_data.len() as f32; // we want to just average this across the dataset
    return result;
}

fn get_training_data() -> Vec<(i32, i32)> {
    return vec![
        //(x, y): y = x * w | solve for w
        (0,0),
        (1,2),
        (2,4),
        (3,6),
        (4,8),
    ];
}
