##### build the image from source code.
FROM rust:1.66-slim as builder

WORKDIR /usr/workspace

COPY . ./

## build
RUN cargo build --release

##### runtime
FROM gcr.io/distroless/cc as runtime

## copy from the builder to runtime
COPY --from=builder /usr/workspace/target/release/hanzlol /

ARG DATABASE_URL
ENV DATABASE_URL ${DATABASE_URL}
EXPOSE 8080

# run the app
CMD ["./hanzlol"]
