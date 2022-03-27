BINARY_NAME=app

build:
	go build -tags netgo -ldflags '-s -w' -o app

run:
	./${BINARY_NAME}

clean:
	go clean
	rm ${BINARY_NAME}
