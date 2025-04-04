# plp owned

Repositório do projeto de Paradigmas de Linguagem de Programação - Sob orientação do Professor Augusto Sampaio, pós graduação em Ciência da Computação - CIn-UFPE 2025.1

### Integrantes
Heitor Sammuel Carvalho Souza | hscs

Bruno da Silva Ramos | bsr

Giovanna Ily Farias Ramalho | gifr

### Introdução
Os integrantes desse grupo não são Javeiros, mas têm muita coragem e um parafuso a menos. Por isso, o projeto da cadeira de PLP 2025.1 será feito em Rust, linguagem que ambos não dominam mas querem se aprofundar. Usando um gerador de parser já existente feito em rust, iremos implementar uma linguagem simples.


### Escopo do Projeto
O escopo do projeto consiste em, utilizando um gerador de parser escrito em Rust:
- Implementar uma linguagem de mesma complexidade e funções da LI1 (Linguagem Imperativa 1) introduzida na cadeira.
- A linguagem criada terá como característica o conceito de _ownership_, também presente na linguagem Rust
  -  Ownership é um conjunto de regras que define como Rust gerencia a memória. Retirado da documentação oficial:
     -  _Cada valor em Rust possui uma variável que é dita seu owner (sua dona)._
     -  _Pode apenas haver um owner por vez._
     -  _Quando o owner sai fora de escopo, o valor será destruído._

#### Parser
Candidatos a parsers para este projeto por enquanto são:
- [lalrpop](https://crates.io/crates/lalrpop)
- [peg](https://crates.io/crates/peg)
- [pest](https://pest.rs/)
- [rust-sitter](https://crates.io/crates/rust-sitter)

## BNF
TBA
