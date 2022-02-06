# WASM, emscripten, threads, and SDL2?

When trying to use SDL2 with `wasm32-unknown-emscripten` from Rust, if the program is compiled for pthreads
(without actually spawning any threads), there is a runtime [exception thrown](https://github.com/gregbuchholz/wasm_sdl_threads/blob/main/exception.txt):

```
worker.js onmessage() captured an uncaught exception: TypeError: SDL2.ctx is undefined
```

...this happens after the first trip through the event loop.  The following
emscription options (see the [.cargo/config](https://github.com/gregbuchholz/wasm_sdl_threads/blob/main/.cargo/config)): 

```
-pthread -sUSE_SDL=2 -s PROXY_TO_PTHREAD -s ASYNCIFY -s ALLOW_MEMORY_GROWTH=1 -s USE_PTHREADS=1 -s PTHREAD_POOL_SIZE=8 -s OFFSCREEN_FRAMEBUFFER=1
```

Compile:
```
cd src/
emcc -c gxx_personality_v0_stub.cpp -pthread
cargo +nightly-2021-12-06 build --target=wasm32-unknown-emscripten --release -Z build-std=panic_abort,std
```

You'll need a very recent emscripten build to get the fix for [this
issue](https://github.com/emscripten-core/emscripten/issues/15891).  There is
also an issue with certain versions of Rust not compiling wasm threaded
programs correctly (especially for `--release`).  The nightly-2021-12-06 is
know to work.

Versions:
```
$ emcc --version
emcc (Emscripten gcc/clang-like replacement + linker emulating GNU ld) 3.1.3-git (a1a755948a6e25c0fa62fc8fdcb89dc372618a63)
Copyright (C) 2014 the Emscripten authors (see AUTHORS.txt)
This is free and open source software under the MIT license.
There is NO warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

```

```
$ rustc +nightly-2021-12-06 --version --verbose
rustc 1.59.0-nightly (e2116acae 2021-12-05)
binary: rustc
commit-hash: e2116acae59654bfab2a9729a024f3e2fd6d4b02
commit-date: 2021-12-05
host: x86_64-unknown-linux-gnu
release: 1.59.0-nightly
LLVM version: 13.0.0
```

There is also a version showing the same problem using C instead of Rust.  That
code is over at
[c_example/](https://github.com/gregbuchholz/wasm_sdl_threads/tree/main/c_example).
See the
[Makefile](https://github.com/gregbuchholz/wasm_sdl_threads/blob/main/c_example/Makefile),
and try running `make run` to see the failing case, and `make run-single` for
the successful case.  The only difference is the arguments passed to `emcc`, with the failing one getting:

```
emcc t_test.c -o t_test.html -pthread -sUSE_SDL=2 -s PROXY_TO_PTHREAD -s ASYNCIFY -s ALLOW_MEMORY_GROWTH=1 -s USE_PTHREADS=1 -s PTHREAD_POOL_SIZE=8 -s OFFSCREEN_FRAMEBUFFER=1
```

