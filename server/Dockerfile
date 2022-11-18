# Please build in parent dir

FROM rust:1.65.0 as build

RUN mkdir /app
RUN mkdir /shared
WORKDIR /app
COPY ./server .
COPY ./shared ../shared
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/local/cargo/bin/server /usr/local/bin/server

CMD ["server"]
