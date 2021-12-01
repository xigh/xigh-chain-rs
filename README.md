### simple naive blockchain implementation in Rust

Based on these very interesting [videos on youtube](https://www.youtube.com/watch?v=zVqczFZr124)

You can find the JavaScript implementation here :

[https://github.com/Savjee/SavjeeCoin](https://github.com/Savjee/SavjeeCoin)

### usage

```bash
git clone https://github.com/xigh/xigh_chain_rs
cd xigh_chain_rs
cargo run --example hello
```

This should print the following :

```text
> cargo run --example hello
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target\debug\examples\hello.exe`
### computing hash for index 0
add vecs:
### computing hash for index 1
### computing hash for index 2
block: index         = 0
       timestamp     = 2021-12-01 18:05:03.643040400 UTC
       data          = []
       previous_hash =
       hash          = af85b77a6122c74044472d0510c1aa605d5d47072aa80334b972fae9e75c9204

block: index         = 1
       timestamp     = 2021-12-01 18:05:03.643350800 UTC
       data          = [0, 1, 2]
       previous_hash = af85b77a6122c74044472d0510c1aa605d5d47072aa80334b972fae9e75c9204
       hash          = 151bff8d79f975c821795d95d2b4c5762334102e5875275af1c55ce0958f96eb

block: index         = 2
       timestamp     = 2021-12-01 18:05:03.643468900 UTC
       data          = [3, 4, 5, 6]
       previous_hash = 151bff8d79f975c821795d95d2b4c5762334102e5875275af1c55ce0958f96eb
       hash          = 8a274a530891090447f8cfc2eb12fd2ad7c6af506b9c680e0b2bc9fe363f1254

### computing hash for index 1
### computing hash for index 2
is_valid true
trying to tamper data:
### computing hash for index 1
block 1 has invalid hash 820dc627f2fa2b182ccddbbb16554f9dca5750d6f24a993842aa8e95aa2589bb
is_valid false
```
