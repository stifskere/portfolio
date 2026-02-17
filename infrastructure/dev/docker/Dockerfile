FROM nim65s/cargo-binstall AS installer

# Install Build and Runtime Dependencies
RUN cargo binstall -y --locked \
	trunk \
	just \
	watchexec-cli

# Download atlasgo for migration apply-on-edit.
RUN curl -sSfL https://atlasgo.sh | sh




FROM rust:1.91-bookworm AS runner

# Set build time parameters.
ARG PF_AUTO_MIGRATE=false

# Define environment variables.
ENV PF_AUTO_MIGRATE=${PF_AUTO_MIGRATE}
ENV DEBIAN_FRONTEND=nointeractive

# Setup the user that will run the project.
RUN useradd -m dev
WORKDIR /home/dev/portfolio

RUN mkdir target dist
RUN chown -R dev:dev .

USER dev

# Setup the PATH and move Dependencies.
ENV PATH="/home/dev/.cargo/bin:$PATH"
ENV PATH="/usr/local/bin/:$PATH"

COPY --from=installer /usr/local/cargo/bin/trunk /home/dev/.cargo/bin/trunk
COPY --from=installer /usr/local/cargo/bin/just /home/dev/.cargo/bin/just
COPY --from=installer /usr/local/cargo/bin/watchexec /home/dev/.cargo/bin/watchexec
COPY --from=installer /usr/local/bin/atlas /usr/local/bin/atlas

# Add the target to run the project
RUN rustup target add wasm32-unknown-unknown

EXPOSE 8080

# Run the both the backend and frontend as specified in the dev recipe.
CMD ["just", "dev"]
