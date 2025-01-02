# Problem

1. run command `npx wrangler dev` in your terminal

2. see error in terminal, must be like this:

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

3. if you comment this code in `src/lib.rs` errors gone, and you can see the `Hello, World!`  when open
   `http://localhost:8787`

```rust
// comment this code on line 22
let _res = reqwest::get("https://httpbin.org/ip").await.unwrap();
```

# Solution

¯\_(ツ)_/¯