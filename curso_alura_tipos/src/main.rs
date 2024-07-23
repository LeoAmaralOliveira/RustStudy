fn main() {
    let notas: [f32; 4] = [10.0, 8.0, 9.5, 6.0];

    for indice in 0..notas.len() {
        println!("A nota {} é = {}", indice + 1, notas[indice])
    }

    println!();
    
    matriz();
    
    println!();

    eh_dia_da_semana(DiaDaSemana::Segunda);
    eh_dia_da_semana(DiaDaSemana::Terca);
    eh_dia_da_semana(DiaDaSemana::Quarta);
    eh_dia_da_semana(DiaDaSemana::Quinta);
    eh_dia_da_semana(DiaDaSemana::Sexta);
    eh_dia_da_semana(DiaDaSemana::Sabado);
    eh_dia_da_semana(DiaDaSemana::Domingo);

    println!();

    cores();

    println!();

    conteudo_opcional();

    println!();

    vectors();

    println!();

    conta_corrente();
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RGBColor(u8, u8, u8),
    CYMKColor{cyan: u8, yellow: u8, magenta: u8, black: u8}
}

fn cores() {
    let cor = Color::CYMKColor{cyan: 1, yellow: 2, magenta: 3, black: 1};
    println!("Cor = {}", match cor {
            Color::Red => "Vermelho",
            Color::Green => "Verde",
            Color::Blue => "Blue",
            Color::RGBColor(0, 0, 0)
                | Color::CYMKColor{
                    cyan: _,
                    yellow: _,
                    magenta: _,
                    black: 255
                }=> "Preto",
            Color::RGBColor(255, 255, 255) => "Branco",
            Color::RGBColor(_, _, _) => "RGB desconhecido",
            Color::CYMKColor{cyan: _, yellow: _, magenta: _, black: _} => "CYMK desconhecido"
        }
    )
}

enum DiaDaSemana {
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
    Domingo
}

fn eh_dia_da_semana(dia_da_semana: DiaDaSemana) {
    println!("{}", match dia_da_semana {
            DiaDaSemana::Sabado | DiaDaSemana::Domingo => "É final de semana",
            _ => "Ainda não é final de semana"
        }
    )
}

fn matriz() {
    let notas: [[f32; 4]; 2] = [
        [1.0, 1.5, 2.0, 2.5],
        [3.0, 3.5, 4.0, 4.5]
    ];

    for linha in notas {
        for nota in linha {
            println!("Nota {}", nota)
        }
    }
}

#[allow(unused_variables)]
fn conteudo_opcional(){
    let conteudo_arquivo = ler_arquivo(String::from(""));

    match &conteudo_arquivo {
        Some(valor) => println!("{}", valor),
        None => println!("Arquivo nao existe")
    };

    println!("{:?}", conteudo_arquivo);

    if let Some(valor) = conteudo_arquivo {
        println!("Tem valor");
    }
}

#[allow(unused_variables)]
fn ler_arquivo(caminho: String)-> Option<String>{
    Some(String::from("Arquivo lido"))
}

fn vectors(){
    let mut notas: Vec<f32> = Vec::with_capacity(4);

    notas.push(10.0);
    println!("Capacidade {}", notas.capacity());
    notas.push(8.8);
    println!("Capacidade {}", notas.capacity());
    notas.push(6.5);
    println!("Capacidade {}", notas.capacity());
    notas.push(6.5);
    println!("Capacidade {}", notas.capacity());
    notas.push(6.5);
    println!("Capacidade {}", notas.capacity());

    println!("{:?}", notas);
    println!("{:?}", notas[0]);
    println!("{:?}", notas.get(7));

    println!("");

    println!("Nota: {}", match notas.get(3) {
        Some(n) => *n,
        None => 0.0
    });

    println!("");

    for i in &notas {
        println!("Nota: {}", i);
    }

    while let Some(nota) = notas.pop() {
        println!("Valor removido: {}", nota);
    }
    println!("{:?}", notas);

    notas.push(10.0);
    notas.push(8.8);
    notas.push(6.5);


}

struct Conta {
    titular: Titular,
    saldo: f64
}

struct Titular {
    nome: String,
    sobrenome: String
}

impl Conta {
    fn sacar(&mut self, valor: f64){
        self.saldo -= valor;
    }
}

fn conta_corrente(){
    let titular = Titular{
        nome: String::from("Leonardo"),
        sobrenome: String::from("Amaral")
    };
    let mut conta: Conta = Conta{
        titular,
        saldo: 100.0
    };

    println!("Dados da conta: Titular = {} {}, Saldo = {}", conta.titular.nome, conta.titular.sobrenome, conta.saldo);

    conta.sacar(50.0);

    println!("Dados da conta: Titular = {} {}, Saldo = {}", conta.titular.nome, conta.titular.sobrenome, conta.saldo);
}