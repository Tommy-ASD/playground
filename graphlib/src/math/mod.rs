use ordered_float::OrderedFloat;

// k is const
fn af(k: OrderedFloat<f64>, x: OrderedFloat<f64>) -> OrderedFloat<f64> {
    let k2 = k;
    x * x / k2
}

fn af_with_weight(
    k: OrderedFloat<f64>,
    x: OrderedFloat<f64>,
    weight: OrderedFloat<f64>,
) -> OrderedFloat<f64> {
    let k2 = k;
    x * x / k2 * weight
}

// k is const
fn rf(k: OrderedFloat<f64>, z: OrderedFloat<f64>) -> OrderedFloat<f64> {
    let k2 = k;
    -k2 * k2 / z
}

fn get_const_c(width: OrderedFloat<f64>) -> OrderedFloat<f64> {
    let c = width / 10.0;
    c
}

fn cool(t: OrderedFloat<f64>) -> OrderedFloat<f64> {
    if t > ordered_float::OrderedFloat(0.001) {
        t * 0.99
    } else {
        ordered_float::OrderedFloat(0.001)
    }
}

fn f_rand(f_min: OrderedFloat<f64>, f_max: OrderedFloat<f64>) -> OrderedFloat<f64> {
    let f = OrderedFloat(rand::random::<f64>());
    f_min + f * (f_max - f_min)
}
