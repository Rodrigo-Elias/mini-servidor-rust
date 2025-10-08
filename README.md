# 🚀 Mini Servidor HTTP de Arquivos Estáticos com Rust

![Rust Logo](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=white)
![Tokio](https://img.shields.io/badge/Tokio-42C8F8?style=for-the-badge&logo=async-await&logoColor=black)
![HTTP](https://img.shields.io/badge/Protocol-HTTP%201.1-28A745?style=for-the-badge)

Um servidor HTTP assíncrono básico, construído manualmente com **Rust** e o *runtime* **Tokio**. O projeto demonstra a capacidade de lidar com requisições de rede, roteamento simples e servir arquivos estáticos de forma eficiente.

## 🌟 Destaques Técnicos

Este projeto foca em conceitos essenciais da programação de sistemas e rede:

* **Assincronicidade com Tokio:** Utiliza `tokio::spawn` para processar cada conexão em uma tarefa separada, garantindo **concorrência** e alta responsividade.
* **Networking Manual:** Implementação do servidor TCP (`TcpListener`) e parsing manual da requisição HTTP (método e path).
* **Roteamento e I/O:** Roteamento de caminho (`/index.html`, `/style.css`) e leitura de arquivos (`tokio::fs::read`) sem bloquear o *thread* principal.
* **Identificação MIME:** Uso do `mime_guess` para enviar o cabeçalho `Content-Type` correto ao navegador (ex: `text/html`, `text/css`).

## 🛠️ Como Rodar o Projeto

### Pré-requisitos

Certifique-se de ter o **Rust** (com Cargo) e o **Git** instalados.

1.  **Clone o Repositório:**
    ```bash
    git clone [SUA URL DO REPOSITÓRIO AQUI]
    cd nome-do-seu-projeto
    ```

2.  **Crie a Pasta de Arquivos:**
    O servidor espera que os arquivos estejam na pasta `public/`.
    *(Você pode ignorar esta etapa se os arquivos já estiverem lá.)*

3.  **Execute o Servidor:**
    ```bash
    cargo run
    ```
    Você verá a mensagem: `Servidor rodando em http://127.0.0.1:8080`

### Testando a Funcionalidade

Após iniciar o servidor:

1.  **Home Page (Roteamento /):** Acesse `http://127.0.0.1:8080/` para que o servidor sirva automaticamente o `index.html`.
2.  **Arquivos Estáticos:** Navegue para `/about.html` ou `/style.css` para verificar o roteamento e a correta identificação do `Content-Type`.

## 📂 Estrutura do Código

| Arquivo | Função Principal |
| :--- | :--- |
| `main.rs` | Contém o loop principal (`main`) e a lógica de concorrência com `tokio::spawn`. |
| `handle_connection` | Lida com o parsing da requisição HTTP, extraindo o caminho. |
| `serve_file` | Lógica de roteamento (`/` para `index.html`), leitura do arquivo e construção da resposta HTTP (200 OK ou 404 Not Found). |

---

Feito isso, o seu próximo e último passo no terminal é:

```bash
git add README.md
git commit -m "Adiciona README do projeto com instruções e destaques tecnicos"
git push
