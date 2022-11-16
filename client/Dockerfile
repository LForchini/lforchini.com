FROM rust:latest as build

RUN mkdir /app
RUN mkdir /shared
WORKDIR /app
COPY ./client .
COPY ./shared ../shared
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli
RUN trunk build -d www --release

FROM nginx:latest
COPY --from=build /app/www /var/www/html
COPY --from=build /app/config/nginx.conf /etc/nginx/conf.d/default.conf

CMD ["nginx", "-g", "daemon off;"]