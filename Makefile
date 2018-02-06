NAME := sandbox-db
all: build run

.PHONY: build run gen bench
build:
	docker build -t $(NAME) .

run:
	docker run -t $(NAME)

gen:
	mkfile 256b fixture/256
	mkfile 512b fixture/512
	mkfile 1024b fixture/1024

bench:
	cargo bench

