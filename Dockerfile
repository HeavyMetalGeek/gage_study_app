FROM rust:1.66 as build

RUN rustup update && \
    rustup update && \
    rustc --version && \
    rustup target add wasm32-unknown-unknown && \
    cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src/testapp
COPY . .

RUN cd frontend && trunk build --release
RUN cd backend && cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/testapp/target/release/backend /usr/local/bin/backend 
COPY --from=build /usr/src/testapp/dist /usr/local/bin/dist

WORKDIR /usr/local/bin
CMD ["backend"]
