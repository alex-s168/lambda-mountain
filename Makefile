
test:
	cargo test testsuite

build:
	as -o lm_raw.o BOOTSTRAP/cli.s
	ld -o lm lm_raw.o
	rm lm_raw.o

prod:
	lm -o production.s PRODUCTION/cli.lm
	as -o production.o production.s
	ld -o production   production.o

install:
	lm -o BOOTSTRAP/cli.s BOOTSTRAP/cli.lm
	as -o lm_raw.o BOOTSTRAP/cli.s
	ld -o lm lm_raw.o
	mv lm /usr/local/bin/
	rm lm_raw.o
