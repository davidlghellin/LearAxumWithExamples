[![codecov](https://codecov.io/gh/davidlghellin/LearAxumWithExamples/graph/badge.svg?token=5ZX4BFDXUD)](https://codecov.io/gh/davidlghellin/LearAxumWithExamples)
# Learn Axum

```sh
cargo init
cargo add axum
cargo add tokio
cargo add tokio -F macros -F rt-multi-thread
```

```sh
cargo install cargo-watch
cargo watch -x run
```

```sh
cargo doc
cargo doc --open # en devcontainer falla
```

```sh
cargo add serde
cargo add serde -F derive
```

Para poder obtener los header, añadimos la feature correspondiente
```sh
cargo add axum -F headers
```

## Docker

Para construir en local
```sh
docker build -t     my-rust-app .
docker run   --name myapp -p 3030:3000 -d
docker exec -it     myapp sh
```

## Docker-compose + postgres

```sh
pgcli -h learnaxum_devcontainer-database-1 -p 5432 -U postgres postgres
```
