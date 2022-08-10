use std::io::stdin;

pub fn leer_usize() -> usize {
    let mut input: String = String::new();
    stdin().read_line(&mut input).unwrap();
    // limpiando el input de la terminal
    let input_limpio = input.replace("\n", "").replace("\r", "").parse::<usize>().unwrap();
    input_limpio
}

pub fn leer_f32() -> f32 {
    let mut input: String = String::new();
    stdin().read_line(&mut input).unwrap();
    // limpiando el input de la terminal
    let input_limpio = input.replace("\n", "").replace("\r", "").parse::<f32>().unwrap();
    input_limpio
}

pub fn leer_string() -> String {
    let mut input: String = String::new();
    stdin().read_line(&mut input).unwrap();
    input = input.replace("\n", "").replace("\r", "");
    input
}
