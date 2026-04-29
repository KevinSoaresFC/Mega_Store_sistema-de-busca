mod modelos;
mod motor;

use std::io::{self, Write};
use modelos::Produto;
use motor::SistemaMegaStore;

fn main() {
    let mut sistema = SistemaMegaStore::novo();

    sistema.cadastrar(Produto { id: 1, nome: "Smartphone G5".into(), categoria: "Eletrônicos".into(), preco: 1500.0 });
    sistema.cadastrar(Produto { id: 2, nome: "Capa".into(), categoria: "Acessórios".into(), preco: 50.0 });
    sistema.conectar_produtos("Smartphone G5", "Capa");

    println!("========================================");
    println!("   MEGASTORE - SISTEMA DE BUSCA (V2)    ");
    println!("========================================");

    loop {
        print!("\nDigite o produto (ou 'sair'): ");
        io::stdout().flush().unwrap();
        let mut busca = String::new();
        io::stdin().read_line(&mut busca).unwrap();
        let termo = busca.trim();

        if termo.to_lowercase() == "sair" { break; }

        match sistema.buscar(termo) {
            Some(p) => {
                println!("✅ [{}] {} - R${:.2}", p.categoria, p.nome, p.preco);
                if let Some(recs) = sistema.grafo_recomendacao.get(&termo.to_lowercase()) {
                    println!("⭐ RECOMENDAÇÃO (GRAFO): {:?}", recs);
                }
            },
            None => println!("❌ Produto não encontrado."),
        }
    }
}