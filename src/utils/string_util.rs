use unicode_segmentation::UnicodeSegmentation;

fn round_float(x: f32, precision: usize) -> String {
    format!("{number:.prec$}", number = x, prec = precision)
}

fn unicode_str_len(str: String) -> usize {
    str.graphemes(true).count()
}