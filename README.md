# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## 📝 Descrição do Projeto
Sistema de alta performance desenvolvido em Rust para a MegaStore. O objetivo é realizar buscas instantâneas em catálogos volumosos e sugerir produtos através de um sistema de recomendação baseado em grafos.

## 🛠️ Tecnologias Utilizadas
* **Linguagem:** Rust (Edition 2021)
* **Gerenciador:** Cargo
* **Estruturas:** HashMap (Tabelas Hash) e Vec (Listas de Adjacência para Grafos)
* **Ferramentas de Teste:** cargo test nativo

## 🚀 Como Executar o Sistema
1. Navegue até a pasta: `cd megastore_busca`
2. Compile e execute:
   ```bash
   cargo run

## 🧪 Instruções de como executar os testes
A qualidade e a confiabilidade do código são garantidas por testes automatizados. Para executar os testes unitários (nos módulos) e de integração (na pasta /tests), utilize:

   ```Bash
   cargo test
   ```

## Exemplos de Uso
Ao iniciar o sistema, você pode realizar consultas interativas:

Busca Simples: Digite `Smartphone G5` para obter ID, Categoria e Preço.

Filtro Case-Insensitive: O sistema aceita variações como `smartphone g5` ou `SMARTPHONE G5`.

Sistema de Recomendação: Ao encontrar um item, o sistema sugere produtos relacionados via Grafo (ex: "Quem viu este item também viu: Capa").

Encerramento: Digite `sair` para finalizar a aplicação.

## 🏗️ Arquitetura do Sistema
O sistema foi desenhado de forma modular seguindo as diretrizes de engenharia de software e separação de responsabilidades:

`src/modelos.rs`: Define a estrutura Produto com os atributos ID, Nome, Categoria e Preço.

`src/motor.rs`: Contém a lógica de indexação e o motor de recomendações.

`src/lib.rs`: Atua como a interface de biblioteca que conecta os módulos aos testes externos.

`src/main.rs`: Gerencia a interface com o usuário e o fluxo de entrada/saída.

`tests/`: Pasta dedicada a testes de integração que validam o sistema completo.

## 📊 Algoritmos e Estruturas de Dados Utilizados
O núcleo técnico baseia-se em duas estruturas fundamentais para garantir eficiência:

`Tabelas Hash` (Complexidade O(1)): Utilizamos o HashMap do Rust para mapear nomes de produtos aos seus dados. Isso garante que o tempo de resposta não sofra degradação conforme o catálogo aumenta, realizando buscas em tempo constante.

`Grafos` (Lista de Adjacência): Implementamos um motor de recomendação baseado em grafos, onde os produtos são nós e as relações de interesse são arestas. Isso permite sugerir itens correlacionados de forma lógica e rápida.

## 📈 Considerações sobre Desempenho e Escalabilidade
`Desempenho`: A utilização de Rust e Tabelas Hash permite latências de busca em nível de microssegundos, ideal para o varejo de larga escala.

`Escalabilidade`: A arquitetura modular e o gerenciamento de memória eficiente do Rust permitem que o sistema lide com o crescimento exponencial do catálogo da MegaStore e do número de clientes sem perda de performance.

## 🤝 Contribuições
Este é um projeto acadêmico desenvolvido para a disciplina de Data Structure Strategy and Implementation. Para contribuir, realize um fork, implemente sua melhoria e envie um Pull Request.

## ⚖️ Licença
Este projeto está sob a licença MIT.

`Desenvolvedor`: Kevin Soares Figueiredo Costa

`Instituição`: Centro Universitário UniFECAF
