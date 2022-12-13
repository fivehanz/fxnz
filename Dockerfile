## build the image from source code.
FROM golang:1.18-alpine

WORKDIR /go/src/appdir

COPY . ./

RUN apk --no-cache add make && go mod download && make build && go clean -modcache -r -cache && apk del make 

ENV APP_PORT 8080 
ENV APP_NAME "new_app_name"

# run the app
CMD ["/bin/sh", "-c", "APP_NAME=${APP_NAME} APP_PORT=${APP_PORT} /go/src/appdir/app"]
