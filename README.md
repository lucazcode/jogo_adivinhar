# Jogo de Adivinhação CLI
Jogo de adivinhação de números em linha de comando utilizando **Rust**, desenvolvido para estudo prático dos conceitos iniciais da linguagem.

## Objetivos de Estudo
* **Gestão de Memória**: Uso prático de referências mutáveis e controle de ciclo de vida (*lifetime*) de variáveis.
* **Tratamento de Erros**: Uso de *match* para isolar falhas de digitação de erros de estouro de memória (*overflow*).

* **Otimização com OnceLock**: Inicialização de recursos globais uma única vez, priorizando performance e segurança.

## Arquitetura
A estrutura do projeto foca na separação de responsabilidades:

* **src/main.rs**: Ciclo de vida do aplicativo.
* **src/jogo.rs**: Lógica central incluindo histórico de palpites.
* **src/ui.rs**: Formatação da interface de usuário.
* **src/util.rs**: Feedback dinâmico através de mutação de string.

## Decisões Técnicas
* **Validação Dinâmica**: Uso de tipos intermediários como `i32 (4 bytes)`  para validação do *range* antes da *atribuição* como `u8 (1 byte)`, fornecendo margem para o tratamento adequado da entrada do usuário.

* **Otimização de Alocação**: Reutilização de buffers de strings com  `.clear()`, minimizando alocações na *heap* durante o loop principal.

## Instalação e Execução
O projeto pode ser testado de duas formas:

### 1. Via Executável Direto
Se você deseja apenas jogar sem instalar o ambiente de desenvolvimento:

1. Vá até a seção de [Releases](https://github.com/lucazcode/jogo_adivinhar/releases).
2. Baixe o executável mais recente disponibilizado.
3. Inicie o jogo através do executável.
   * Obs: No Linux/macOS, talvez seja necessário permitir a execução do arquivo:

```bash
chmod +x jogo_adivinhar
./nome_do_binario
```

### 2. Via Cargo (Código Fonte)
Se você possui o Rust instalado, pode compilar e rodar diretamente do código-fonte. O uso da flag `--release` faz com que o compilador aplique todas as otimizações de performance.

```bash
# Clone o repositório
git clone https://github.com/lucazcode/jogo_adivinhar.git
cd jogo_adivinhar

# Compile e rode em modo otimizado
cargo run --release```

## Requisitos
Para garantir o funcionamento correto do projeto, certifique-se de atender aos seguintes requisitos:

### Desenvolvimento
* **Rust Toolchain**: Versão **1.70.0** ou superior (obrigatório).
  * *Motivo*: O projeto utiliza `std::sync::OnceLock`, que foi estabilizado apenas a partir desta versão.
* **Cargo**: Gerenciador de pacotes padrão do Rust.

### Execução
* **Sistemas Operacionais Compatíveis**: 
  * **Linux**
  * **Windows**
  * **macOS**
