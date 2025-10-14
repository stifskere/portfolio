FROM nim65s/cargo-binstall AS installer

RUN cargo binstall -y --locked \
	trunk \
	cargo-watch \
	just

FROM rust:1.89-bookworm

ARG DATABASE_URL

ENV DEBIAN_FRONTEND=nointeractive
ENV DATABASE_URL=${DATABASE_URL}

RUN useradd -m dev
WORKDIR /home/dev/portfolio

RUN mkdir target dist
RUN chown -R dev:dev .

USER dev

ENV PATH="/home/dev/.cargo/bin:$PATH"

COPY --from=installer /usr/local/cargo/bin/cargo-watch /home/dev/.cargo/bin/cargo-watch
COPY --from=installer /usr/local/cargo/bin/trunk /home/dev/.cargo/bin/trunk
COPY --from=installer /usr/local/cargo/bin/just /home/dev/.cargo/bin/just

RUN rustup target add wasm32-unknown-unknown

EXPOSE 8080

CMD just dev
