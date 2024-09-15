Este projeto é um contrato inteligente para gerenciamento de estoque, desenvolvido em Rust utilizando a framework ink! para a blockchain Substrate. O contrato permite a adição de produtos, a atualização de suas quantidades e a consulta de detalhes dos produtos armazenados.

## Como Rodar o projeto
1 - Configuração do Ambiente
* Instalar Rust: Certifique-se de ter o Rust instalado na sua máquina.
* Instale o Substrate.
* Instalar o cargo-contract
* Instalar o ink

2 - Configuração do Projeto
* Clonar o Repositório
* Instalar Dependências (cargo build)

3 - Compilar o Contrato
* Use "cargo contract build" no terminal para compilar o contrato inteligente

4 - Executar Testes
* Execute os testes unitarios digitando "cargo test" no terminal
* Execute os testes de integraçãodigitando "cargo e2e-test" no terminal
  (Para rodar testes de integração, é necessário ter um ambiente de teste que simule a blockchain Substrate. Isso pode ser feito atravésde um nó local Substrate)

