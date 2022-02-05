# WASM, emscripten, threads, and SDL2?

When trying to use SDL2 with `wasm32-unknown-emscripten` from Rust, if the program is compiled for pthreads
(without actually spawning any threads), there is a runtime [exception thrown]():

```
worker.js onmessage() captured an uncaught exception: TypeError: SDL2.ctx is undefined
```

...this happens after the first trip through the event loop.  The following
emscription options (see the [.cargo/config]()): 

```
-pthread -sUSE_SDL=2 -s PROXY_TO_PTHREAD -s ASYNCIFY -s ALLOW_MEMORY_GROWTH=1 -s USE_PTHREADS=1 -s PTHREAD_POOL_SIZE=8 -s OFFSCREEN_FRAMEBUFFER=1
```

