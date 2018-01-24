NAME := sandbox-db
all: build run

.PHONY: build run
build:
	docker build -t $(NAME) .

run:
	docker run -t $(NAME)


