fn main() {
    let f = c_to_f(26.0);
    println!("f = {}", f);

    let c = f_to_c(78.8);
    println!("c = {}", c);
}

fn c_to_f(c: f32) -> f32 {
    let partial_calc : f32 = 9.0 / 5.0;
    let partial_f = c * partial_calc;
    partial_f + 32.0
}

fn f_to_c(f: f32) -> f32 {
    let c = (f - 32.0) * 5.0 / 9.0;
    c
}