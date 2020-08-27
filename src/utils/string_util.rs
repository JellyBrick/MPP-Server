fn round_float(x: f32, precision: usize) -> String {
    format!("{number:.prec$}", number = x, prec = precision)
}