# TaskMaster

TaskMaster é uma API simples de gerenciamento de tarefas (To-Do List) desenvolvida em [Rust](https://www.rust-lang.org/), utilizando a crate [actix-web](https://actix.rs/). A API permite realizar operações CRUD (Create, Read, Update, Delete) para gerenciar tarefas.

## Funcionalidades

- **Criar tarefas**: Adicionar novas tarefas à lista.
- **Listar tarefas**: Obter uma lista completa de todas as tarefas.
- **Visualizar tarefa**: Obter os detalhes de uma tarefa específica através de seu ID.
- **Atualizar tarefas**: Modificar o título e descrição de tarefas existentes.
- **Excluir tarefas**: Remover tarefas pelo ID.

## Endpoints

Aqui estão os endpoints disponíveis nesta API:

- `POST /tasks`: Cria uma nova tarefa.
- `GET /tasks`: Retorna a lista de tarefas.
- `GET /tasks/{id}`: Retorna uma tarefa específica pelo ID.
- `PUT /tasks/{id}`: Atualiza uma tarefa existente.
- `DELETE /tasks/{id}`: Remove uma tarefa pelo ID.

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo (gerenciador de pacotes do Rust)
- [actix-web](https://actix.rs/), [serde](https://serde.rs/), [uuid](https://docs.rs/uuid/1.0.0/uuid/) crates

## Instalação

1. Clone o repositório:
    ```bash
    git clone https://github.com/seu-usuario/taskmaster.git
    cd taskmaster
    ```

2. Instale as dependências:
    ```bash
    cargo build
    ```

3. Execute o projeto:
    ```bash
    cargo run
    ```

4. Acesse a API em `http://localhost:8080`.

## Exemplos de Uso

### Criar uma nova tarefa (POST /tasks)

- Criar o ID e enviar como resposta.
- Usuário não precisa enviar o ID

```bash
curl -X POST http://localhost:8080/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Aprender Rust", "description": "Estudar o básico de Rust"}'
```


### Atualizar uma tarefa (POST /tasks)

- Usuário não precisa enviar todos os campos

```bash
curl -X PUT http://localhost:8080/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Aprender Rust"}'
```


