# Hexale

This is a toy project featuring a two players web based game written in Rust (server) and JavaScript (client).

## Building the client

The client is built using [webpack](https://webpack.js.org).

```bash
npm install
npm run build
```

Running `npm run dev` will watch the files and rebuild the client every time they change.

## Building the server

The server is written in Rust using [IRON](http://ironframework.io). Like most Rust projects, it is built using Cargo.

```bash
cargo run
```

Now you can visit [localhost:3000](http://localhost:3000).
