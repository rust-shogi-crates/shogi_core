.PHONY: tests
tests: position.run

%.x: %.c
	$(CC) $< -o $@ -I../include -D DEFINE_ALLOC=1 -D DEFINE_STD=1 -L../target/release/ -Wl,-rpath ../target/release/ -lshogi_core_c
%.run: %.x
	./$<
