fn main() {

    //Enteros
    let a: i32 = 10;
    let b = 3; // i32 por defecto
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
    println!("a % b = {}", a % b);

    //Flotantes
    let x: f64 = 3.5;
    let y = 2.0;
    println!("x * y = {}", x * y);

    //Booleanos
    let es_mayor = a > b;
    println!("¿a es mayor que b?: {}", es_mayor);

    //Caracteres y cadenas
    let letra = 'R';
    let palabra = "Rust";
    println!("Letra: {}, Palabra: {}", letra, palabra);

    //Mutabilidad
    let mut contador = 0;
    println!("Contador: {}", contador);
    contador += 1;
    println!("Contador: {}", contador);
}
