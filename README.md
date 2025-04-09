# Rust HTTP Server from Scratch

This a http server built from sratch. It's based on expressjs.
It aims to be easy to use

## Current Features

- supports GET, POST, PUT, PATCH, DELETE methods
- It uses concucrrency to handle multiple requests(It should be use an Event Loop like tokyo or async-std)
- It uses serde to parse JSON
- It's able to parse JSON(no supports for other formats yet)

## how to run

### clone project

```
git clone https://github.com/dilanEspindola/http-server.git
```

### run project in hot reload mode

```
 cargo watch -x 'run'
```

### Build optimized app

```
cargo build --release
```

### Run optimized build

```
cargo run --release
```
