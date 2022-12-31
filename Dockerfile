ARG APP_NAME=gage_study_app
FROM rust:1.66 as build

RUN rustup update && \
    rustc --version && \
    rustup target add wasm32-unknown-unknown && \
    cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src/${APP_NAME}
COPY . .

RUN cd frontend && trunk build --release
RUN cd backend && cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/${APP_NAME}/target/release/backend /usr/local/bin/backend 
COPY --from=build /usr/src/${APP_NAME}/dist /usr/local/bin/dist

WORKDIR /usr/local/bin
CMD ["backend"]
