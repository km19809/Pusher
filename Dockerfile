# TUI-enabled docker image, which uses alpine-glibc as runtime os. (~20MB)
FROM rust:latest AS build
WORKDIR /pusher
COPY ./ ./
RUN cargo build --release --features color,tui

FROM frolvlad/alpine-glibc as runtime

COPY --from=build /pusher/target/release/pusher /
COPY --from=build /pusher/stage.data /

CMD [ "/pusher" ]