# wasm-bindgen: failed to import the same js functions with same name under different modules

```sh
$ wasm-pack --version
wasm-pack 0.13.1
```

Both the following tests failed.
In each test, two modules import the same js function under the same name, but with slightly different signatures.

- Different return types

```sh
wasm-pack test --node --test string
```

- Different argument types

```sh
wasm-pack test --node --test f64
```
