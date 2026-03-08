sieve
=====

```
wasm-pack build --target web
python3 -m http.server
## now open http://localhost:8000
```

![Sieve of Eratosthenes](./etc/sieve.gif)

PoC event streaming [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes) from Rust/WebAssembly to JavaScript main/UI thread.

The `mpsc::channel` from [`futures`](https://crates.io/crates/futures) allows bounded non-blocking async channel, that naturally supports back-pressure from main/UI thread.
