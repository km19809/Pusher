# Dockerfile that builds minimal docker image. (~4MB)
# Very slow while building an image.
FROM ekidd/rust-musl-builder:latest AS build
WORKDIR /pusher
COPY ./ ./
RUN cargo build --release --features color,tui

FROM scratch as runtime

COPY --from=build /pusher/target/x86_64-unknown-linux-musl/release/pusher /
COPY --from=build /pusher/stage.data /

CMD [ "/pusher" ]