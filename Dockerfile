##### build the image from source code.
FROM rust:1.66-slim as builder

WORKDIR /usr/workspace

COPY . ./

## install target platform (cross-compilation) -- needed for alpine
RUN rustup target add x86_64-unknown-linux-musl

#RUN apk --no-cache add make && go mod download && make build && go clean -modcache -r -cache && apk del make 

## build
RUN cargo build --target x86_64-unknown-linux-musl --release

##### runtime
FROM alpine:3.17 as runtime

## copy from the builder to runtime
COPY --from=builder /usr/workspace/target/x86_64-unknown-linux-musl/release/hanzlol /usr/local/bin

#ENV APP_PORT 8080 
#ENV APP_NAME "new_app_name"

EXPOSE 8080

# run the app
CMD ["/usr/local/bin/hanzlol"]
