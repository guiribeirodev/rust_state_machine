# Máquina de Estados em Rust

Este projeto implementa uma máquina de estados simples em Rust, inspirada em arquiteturas de blockchain. O sistema permite a gestão de contas, transferências de saldo e prova de existência de dados. Foi realizado durante o evento da Web3Dev <https://build.w3d.community/courses/Rust_State_Machine>

## Visão Geral

O projeto é estruturado em módulos (pallets) que implementam diferentes funcionalidades:

- **Sistema**: Gerencia números de blocos e nonces de contas
- **Saldos**: Controla transferências e saldos entre contas
- **Prova de Existência**: Permite que usuários reivindiquem e revoguem propriedade sobre conteúdos

## Como Executar

Para construir e executar o projeto:

```bash
cargo build
cargo run
```

## Funcionalidades

### Saldos e Transferências

O módulo `balances` permite:

- Verificar saldos de contas
- Definir saldos iniciais
- Transferir valores entre contas com verificações de saldo suficiente e overflow

### Prova de Existência

O módulo `proof_of_existence` permite:

- Criar reivindicações de propriedade sobre conteúdos
- Revogar reivindicações existentes
- Verificar o proprietário de um conteúdo

### Sistema

O módulo `system` gerencia:

- Números de blocos
- Nonces de transações para cada conta

## Arquitetura

O projeto utiliza macros de procedimento para gerar código que facilita:

1. A criação de chamadas entre módulos
2. A configuração do runtime principal

O fluxo de execução segue um modelo similar a blockchains:

1. Formação de blocos com transações (extrinsics)
2. Execução de blocos em sequência
3. Despacho de chamadas para os módulos apropriados

## Extensão

Para adicionar novos módulos:

1. Crie um novo arquivo de módulo em src
2. Implemente a trait `Config` para definir tipos associados
3. Crie uma estrutura `Pallet<T: Config>` com a lógica do módulo
4. Use a macro `#[macros::call]` para expor funções chamáveis
5. Adicione o novo módulo à estrutura `Runtime` em main.rs

## Exemplos

Veja o arquivo main.rs para exemplos de como criar blocos e executá-los no runtime.
