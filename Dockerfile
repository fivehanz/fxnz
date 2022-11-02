## build the image from source code.
FROM golang:1.18-alpine

WORKDIR /go/src/appdir

COPY . ./

RUN go mod download

RUN apk --no-cache add make

RUN make build

# TODO: env not working when running 
ENV APP_PORT 8080 
ENV APP_NAME "app_name"

EXPOSE 8080

# run the app
CMD ["/bin/sh", "-c", "APP_NAME=${APP_NAME} APP_PORT=${APP_PORT} /go/src/appdir/app"]
