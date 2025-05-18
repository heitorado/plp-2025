# PLP owned (Rust Ownership)

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
  - Ownership é um conjunto de regras que define como Rust gerencia a memória. Retirado da documentação oficial:
    - _Cada valor em Rust possui uma variável que é dita seu owner (sua dona)._
    - _Pode apenas haver um owner por vez._
    - _Quando o owner sai fora de escopo, o valor será destruído._

#### Parser

A biblioteca de construção de parsers, [nom](https://docs.rs/nom/latest/nom/), é um componente essencial do projeto, responsável por processar tokens conforme as regras gramaticais da linguagem. Ela permite a composição modular de parsers, facilitando a criação de soluções complexas a partir de partes menores e reutilizáveis. Com foco em produtividade e clareza, a ferramenta simplifica a implementação de regras sintáticas, promove organização do código e oferece flexibilidade para expansões futuras.

- Função: Consumir tokens e validar estruturas sintáticas.

- Destaque: Arquitetura modular que favorece reuso e manutenção.

## BNF da Linguagem Imperativa 2 com implementação de funções como expressão.

```
Programa ::= Comando

Comando ::= Atribuicao
            | ComandoDeclaracao
            | While
            | IfThenElse
            | IO
            | Comando ";" Comando
            | Skip
            | ChamadaProcedimento

Skip ::=

Atribuicao ::= Id ":=" Expressao

Expressao ::= Valor | ExpUnaria | ExpBinaria | Id | ChamadaProcedimento

Valor ::= ValorConcreto

ValorConcreto ::= ValorInteiro | ValorBooleano | ValorString

ExpUnaria ::= "-" Expressao | "not" Expressao | "length" Expressao

ExpBinaria ::= Expressao "+" Expressao
            | Expressao "-" Expressao
            | Expressao "and" Expressao
            | Expressao "or" Expressao
            | Expressao "==" Expressao
            | Expressão "++" Expressao
            | Expressao "<=" Expressao
            | Expressao ">=" Expressao
            | Expressao "<" Expressao
            | Expressao ">" Expressao


ComandoDeclaracao :: = "{" Declaracao ";" Comando "}"

Declaracao ::= DeclaracaoVariavel |  DeclaracaoComposta | DeclaracaoProcedimento

DeclaracaoVariavel ::= "var" Id "=" Expressao

DeclaracaoComposta ::= Declaracao "," Declaracao

DeclaracaoProcedimento ::= "proc" Id "(" ListaDeclaracaoParametro ")" "{" Comando "}"

ListaDeclaracaoParametro ::= Tipo Id | Tipo Id "," ListaDeclaracaoParametro

Tipo ::= "string" | "int" | "boolean"

While ::= "while" Expressao "do" Comando

IfThenElse ::= "if" Expressao "then" Comando "else" Comando

IO ::= "write" "(" Expressao ")" | "read" "(" Id ")"

ChamadaProcedimento ::= "call" Id "(" [ListaExpressao] ")"

ListaExpressao ::= Expressao | Expressao, ListaExpressao
```
