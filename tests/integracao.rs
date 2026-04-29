use megastore_busca::motor::SistemaMegaStore;
use megastore_busca::modelos::Produto;

#[test]
fn teste_completo_sistema() {
    let mut sistema = SistemaMegaStore::novo();
    let p = Produto { id: 1, nome: "Teste".into(), categoria: "Geral".into(), preco: 10.0 };
    
    sistema.cadastrar(p);
    sistema.conectar_produtos("Teste", "Relacionado");

    
    assert!(sistema.buscar("teste").is_some());
    
    
    let recs = sistema.grafo_recomendacao.get("teste").unwrap();
    assert!(recs.contains(&"Relacionado".to_string()));
}