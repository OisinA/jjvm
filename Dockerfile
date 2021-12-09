FROM rust:1.57 as build

WORKDIR /usr/src/jjvm

COPY . .

RUN cd jjvm && cargo build --release

FROM gcr.io/distroless/cc

COPY --from=build /usr/src/jjvm/jjvm/target/release/jjvm /usr/bin/jjvm

ENTRYPOINT [ "jjvm" ]