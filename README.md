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

## BNF da Linguagem Imperativa 1 com implementação de funções.

```
Programa ::= Comando

Comando ::= Atribuicao

            | ComandoDeclaracao

            | While

            | IfThenElse

            | IO

            | Comando ";" Comando

            | "return" Expressao // Modificado para adicionar função

            | Skip

Skip ::=

Atribuicao ::= Id ":=" Expressao

Expressao ::= Valor | ExpUnaria | ExpBinaria | Id | ChamadaFuncao // Modificado para adicionar função

Valor ::= ValorConcreto

ValorConcreto ::= ValorInteiro | ValorBooleano | ValorString

ExpUnaria ::= "-" Expressao | "not" Expressao | "length" Expressao

ExpBinaria ::= Expressao "+" Expressao

            | Expressao "-" Expressao

            | Expressao "and" Expressao

            | Expressao "or" Expressao

            | Expressao "==" Expressao

            | Expressao "++" Expressao

ListaParametros ::= Expressao "," ListaParametros | Expressao | Skip // Modificado para adicionar função

ChamadaFuncao ::= Id "(" ListaParametros ")" // Modificado para adicionar função

ComandoDeclaracao :: = "{" Declaracao ";" Comando "}"

Declaracao ::= DeclaracaoVariavel |  DeclaracaoComposta | DeclaracaoFuncao // Modificado para adicionar função

DeclaracaoVariavel ::= "var" Id "=" Expressao

DeclaracaoComposta ::= Declaracao "," Declaracao

DeclaracaoFuncao ::= "func" Id "(" Parametros ")" "{" Comando "}" // Modificado para adicionar função

While ::= "while" Expressao "do" Comando

IfThenElse ::= "if" Expressao "then" Comando "else" Comando

IO ::= "write" "(" Expressao ")" | "read" "(" Id ")"
```

## Ideia de implementação do Ownership

```
// Parte de Atribuição
// Adiciona 'move' como opcional
// Com o 'move' a posse da expressão é atribuida ao novo X e o id antigo deixaria de existir.
// Sem o 'move' a atribuição gera uma copia da expressão para o novo ID.
Atribuicao ::= Id ":=" "move"? Expressao

Exemplo:
    var a = 2;
    var b = move a; // posse de a é transferido para b
    write(b); // Valido
    write(a); // Inválido (erro) pois a perdeu a posse.

// Parte do escopo
// Implicitamente os recursos serão liberado ao sair do escopo (os recursos só são validos dentro de seus respectivos escopos.)
ComandoDeclaracao ::= "{" Declaracao ";" Comando "}" // Sem alteração na BNF

Exemplo:
    {
        var a = 10; // definição da variável.
        write(a);
    } // delimitação do escopo, variáveis liberadas.

    write(a); // Inválido (erro) a variável não exite mais.
```
