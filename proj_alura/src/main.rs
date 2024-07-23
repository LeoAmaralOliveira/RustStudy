fn main() {
    ownership();
    pattern_matching();
    erros1();
    erros2();
}

fn ownership() {
    let mut uma_string = String::from("Teste");
    rouba(&mut uma_string);
    println!("{}", uma_string)
}

fn rouba(string: &mut String) {
    println!("{}", string);
    string.push_str(" Teste2");
    println!("{}", string)
}

fn pattern_matching() {
    for x in 1..=20 {
        println!("{}: {}", x, match x {
            1..=9 => "Pouco",
            10 => "Meio",
            11..=20 => "Muito",
            _ => "Não encontrado"
        });
    }
}

fn erros1() {
    panic!("Erro detectado");
}

fn erros2() {
    match result() {
        Ok(s) => println!("String OK -> {}", s),
        Err(n) => println!("Erro code -> {}", n)
    }
}

fn result() -> Result<String, u16> {
    Ok(String::from("Deu bom"))
    // Err(404)
}