BINARY_NAME=app

build:
	go build -tags netgo -ldflags '-s -w' -o app

run:
	./${BINARY_NAME}

clean:
	go clean
	rm ${BINARY_NAME}

build-image:
	docker buildx build -t ghcr.io/fivehanz/fxnz:latest . --platform linux/amd64

push-image:
	docker push ghcr.io/fivehanz/fxnz:latest