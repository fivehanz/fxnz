## Step 1: build the image from source code.
FROM golang:1.17-alpine as builder

WORKDIR /go/src/appdir

COPY . ./

RUN go mod download

RUN apk --no-cache add make

RUN make build

## Step 2: create image for deployment
# fresh image for minimal size.
FROM scratch

COPY --from=builder /go/src/appdir/app /appdir/app

# run the app
CMD ["/appdir/app"]
