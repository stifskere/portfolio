# Building And Development

As already mentioned in the [readme file](/README.md), this "portfolio" is more
of an ecosystem entry than a portfolio. Meaning that its structure
is more complex than a single container application.

## Getting Started

The dependencies to develop this project are very straightforward.

This project uses [`just`](https://just.systems/main/en) as a recipe runner
to trigger the development and build procedures for this application,
it's tightly integrated with docker, as it is configured
to detect whether you are running it from inside or outside of a
container.

You need to install
- `just`: The recipe runner from [just.systems](https://just.systems)
- `docker`: You need `docker`, `docker-compose` and `buildx` setup, `containerd` is only tested.
- `rust`: You need `rustc` installed and `cargo metadata` for deployment checks, targets are configured in docker.

You'd need `terraform` too if you are willing to deploy this from your machine. But it's not meant to
be this way.

There are two main recipes to the `justfile`.

### `@dev`

This recipe will trigger a production build, start the containers
declared in [`docker/dev.docker-compose.yml`] and attach them to
the current shell.

It will use the port `8080` and `8081` for both the back-end
and front-end, even tho the main entry point is `8080` and
the back-end can be accessed by a proxy rewrite @ `/api/*`

> [!IMPORTANT]
> By default both ends bind @ `0.0.0.0`.

### `@build`

This recipe will make a production build with an `$IMAGE_NAME`
of preference or `portfolio` as coalescing value, the tag will
be `prod-$VERSION` where `$VERSION` is the evaluation of all the crates
in the workspace.

> [!WARNING]
> The build process will halt if all the crates in the workspace don't
> share the same version.

## Deployment

The project deployment has a very specific terraform configuration, even tho
that configuration is very variable, I cannot assure that you won't find
something you need to change if you wish to deploy this by yourself.

The `build-and-push` workflow is also very specific, as it assumes
you will upload this to `ghcr.io` and the container name will be
`stifskere/portfolio` which is where all of this code is hosted.
