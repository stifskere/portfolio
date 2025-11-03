FROM nim65s/cargo-binstall AS installer

# Install build dependencies.
RUN cargo binstall -y --locked \
	trunk \
	just




FROM rust:1.91-slim-bookworm AS builder

# Setup justfile flag.
ARG BUILD_STAGE
ENV BUILD_STAGE=$BUILD_STAGE

# Copy build dependencies.
ENV PATH="/home/prod/.cargo/bin:$PATH"

COPY --from=installer /usr/local/cargo/bin/trunk /home/prod/.cargo/bin/trunk
COPY --from=installer /usr/local/cargo/bin/just /home/prod/.cargo/bin/just


# Setup user to match running user.
RUN useradd -m prod
RUN chmod 1777 /tmp

# Move into application directory.
WORKDIR /home/prod/app
RUN chown -R prod:prod /home/prod

USER prod

# Setup user environment variables
ENV HOME=/home/prod
ENV CARGO_HOME=/home/prod/.cargo
ENV RUSTUP_HOME=/home/prod/.rustup

# Copy project into application directory.
COPY --chown=prod:prod . .

# Add target and build project.
RUN rustup default stable
RUN rustup target add wasm32-unknown-unknown
RUN just build




FROM python:3.13-slim-bookworm AS runner

# Copy built application.
COPY --chown=prod:prod --from=builder /home/prod/app/dist /home/prod/app/

# Setup running user.
RUN useradd -m prod

# Move into application directory.
WORKDIR /home/prod/app
RUN chown -R prod:prod /home/prod

USER prod

# Setup user environment variables
ENV HOME=/home/prod

# Expose and run built application.
EXPOSE 8080
CMD [ \
	"python3", \
	"-u", \
	"-m", "http.server", \
	"8080", \
	"--bind", "0.0.0.0", \
	"-d", "." \
]
