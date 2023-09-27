# 2º Trabalho Prático de Sistemas Distribuidos (COS-470 - de Eng. da Computação e Informação - UFRJ)

Implementação de problema do consumidor-produtor e de somatório com primitivas de sincronização. No primeiro, o lock utilizado foi implementado do zero, enquanto os semáforos da segunda parte são pertencentes às bibliotecas padrão do Rust.

## Instalação do Rust

- Linux ou macOS: Abra o terminal e execute o comando abaixo:

  ```shell
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- Windows:
  1. Visite o site oficial do Rust em [https://www.rust-lang.org/](https://www.rust-lang.org/).
  2. Baixe e execute o instalador do Rustup.

````

## Compilando...

1. Vá no diretório do projeto via terminal e digite:

   ```shell
   cargo run
````

- Se houver dependências no arquivo `Cargo.toml`, o Cargo irá resolvê-las automaticamente.

5. Para compilar o projeto sem executá-lo, use o comando `cargo build`.
