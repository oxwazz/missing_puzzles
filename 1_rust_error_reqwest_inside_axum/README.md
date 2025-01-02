# Problem

1. Run command `npx wrangler dev` in your terminal

2. See error in terminal. Must be like this:

```shell
...
error[E0277]: the trait bound `fn() -> impl Future<Output = &'static str> {index}: Handler<_, _>` is not satisfied
   --> src/lib.rs:7:34
    |
7   | ...e("/", any(index))
    |           --- ^^^^^ the trait `Handler<_, _>` is not implemented for fn item `fn() -> impl Future<Output = ...> {index}`
    |           |
    |           required by a bound introduced by this call
    |
    = note: the full name for the type has been written to ...
...
```

3. You can comment this code in `src/lib.rs` and the error is gone, and you can see the `Hello, World!` text when open
   `http://localhost:8787` in your browser

```rust
// comment this code on line 22
let _res = reqwest::get("https://httpbin.org/ip").await.unwrap();
```

# Solution

¯\_(ツ)_/¯