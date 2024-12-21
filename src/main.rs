use std::io::{self, Write};

fn main() {
    let mut translate = String::new();

    print!("Translate: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut translate).expect("Erro ao ler a variável translate");

    println!("Translation: (Mensagem Traduzida)");
}
