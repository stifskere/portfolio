FROM nim65s/cargo-binstall AS installer

# Install Build and Runtime Dependencies
RUN cargo binstall -y --locked \
	trunk \
	cargo-watch \
	just




FROM rust:1.91-bookworm AS runner

# Receive Parameters from Docker Compose.
ARG DATABASE_URL

ENV DEBIAN_FRONTEND=nointeractive
ENV DATABASE_URL=${DATABASE_URL}

# Setup the user that will run the project.
RUN useradd -m dev
WORKDIR /home/dev/portfolio

RUN mkdir target dist
RUN chown -R dev:dev .

USER dev

# Setup the cargo path and move Dependencies.
ENV PATH="/home/dev/.cargo/bin:$PATH"

COPY --from=installer /usr/local/cargo/bin/cargo-watch /home/dev/.cargo/bin/cargo-watch
COPY --from=installer /usr/local/cargo/bin/trunk /home/dev/.cargo/bin/trunk
COPY --from=installer /usr/local/cargo/bin/just /home/dev/.cargo/bin/just

# Add the target to run the project
RUN rustup target add wasm32-unknown-unknown

EXPOSE 8080

# Run the both the backend and frontend as specified in the dev recipe.
CMD just dev
