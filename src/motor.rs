use std::collections::HashMap;
use crate::modelos::Produto;

pub struct SistemaMegaStore {
    pub catalogo: HashMap<String, Produto>,
    pub grafo_recomendacao: HashMap<String, Vec<String>>,
}

impl SistemaMegaStore {
    pub fn novo() -> Self {
        Self {
            catalogo: HashMap::new(),
            grafo_recomendacao: HashMap::new(),
        }
    }

    pub fn cadastrar(&mut self, p: Produto) {
        self.catalogo.insert(p.nome.to_lowercase(), p);
    }

    pub fn conectar_produtos(&mut self, p1: &str, p2: &str) {
        self.grafo_recomendacao.entry(p1.to_lowercase()).or_insert(vec![]).push(p2.to_string());
        self.grafo_recomendacao.entry(p2.to_lowercase()).or_insert(vec![]).push(p1.to_string());
    }

    pub fn buscar(&self, nome: &str) -> Option<&Produto> {
        self.catalogo.get(&nome.to_lowercase())
    }
}