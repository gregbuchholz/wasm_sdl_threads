.PHONY: threaded
threaded: t_test.js

t_test.js: t_test.c
	emcc t_test.c -o t_test.html -pthread -sUSE_SDL=2 -s PROXY_TO_PTHREAD -s ASYNCIFY -s ALLOW_MEMORY_GROWTH=1 -s USE_PTHREADS=1 -s PTHREAD_POOL_SIZE=8 -s OFFSCREEN_FRAMEBUFFER=1

.PHONY: single-threaded
single-threaded: t_test_single.js

t_test_single.js: t_test.c
	emcc t_test.c -o t_test_single.html -s USE_SDL=2 -s ASYNCIFY -s ALLOW_MEMORY_GROWTH=1

.PHONY: run
run: t_test.js
	emrun index_t_test.html

.PHONY: run-single
run-single: t_test_single.js
	emrun index_t_test_single.html

.PHONY: all
all: threaded single-threaded

.PHONY: clean
clean:
	rm -f t_test.js t_test.worker.js t_test.wasm t_test.html
	rm -f t_test_single.js t_test.worker_single.js t_test_single.wasm t_test_single.html

