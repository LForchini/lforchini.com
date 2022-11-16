FROM rust:1.65.0

RUN mkdir /app
RUN mkdir /shared
WORKDIR /app
COPY ./server .
COPY ./shared ../shared
RUN cargo build --release

CMD ["cargo",  "run", "--release"]
