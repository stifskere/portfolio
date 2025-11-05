set dotenv-load

just := `command -v just`

# just dev
#
# Starts a dvelopment container.
#
# Triggers `docker compose` to build and start a container, the definitions
# are at `docker/dev.docker-compose.yml`.
@dev:
	#!/bin/bash
	set -e;

	if
		[[ -f /.dockerenv ]] ||
		grep -qE '(docker|containerd|kubepods)' /proc/1/cgroup 2>/dev/null;
	then
		cargo watch -x 'run -p backend' 2>&1 & :;
		trunk serve --config Trunk.toml 2>&1 & :;
		wait;
	else
		if !command -v docker >/dev/null 2>&1; then
			echo "Please, install a linux compatible version of docker before continuing.";
			exit 1;
		fi

		docker compose -f ./docker/dev.docker-compose.yml up --no-deps --build;
	fi


# just build
#
# Builds the project or provides build information.
#
# This recipe will build a container with the tag
# "$IMAGE_NAME" or `portfolio` if the variable is not
# found.
#
# Building will output the tag to stdout.
@build:
	#!/bin/bash
	set -e;

	if [ "$BUILD_STAGE" = "true" ]
	then
		trunk build --release;
	else
		image_name="${IMAGE_TAG:-portfolio}";
		version="$({{just}} info version)";

		docker build --build-arg BUILD_STAGE=true -t "$image_name:$version" -f ./docker/prod.dockerfile .;
		echo "$image_name:$version";
	fi

# just info <key>
#
# Provides information about the project.
#
# The info is interpreted as key value pairs, where accessing
# a key will compute the value and display it on stdout.
#
# In the case a value can't be computed the exit code will be 1,
# meaning that the output cannot be trusted.
#
# Allowed keys:
# - `version`: Computes the version of the project.
@info key:
	#!/bin/bash
	set -e;

	function version() {
		local version=$(
			cargo metadata --format-version 1 --no-deps \
			| jq -r '[.packages[].version] | if (unique | length) == 1 then .[0] else empty end'
		);

		if [ -z "$version" ]
		then
			echo "Versions differ";
			exit 1;
		fi

		echo "$version";
	}

	case "{{key}}" in
		"version")
			version
			;;
		*)
			echo "Key not found."
			exit 1
			;;
	esac


# PENDING TO MIGRATE.
@migrate:
	#!/bin/bash
	set -e;

	rm -f migrations/.__schema.sql;
	cat migrations/*.sql > migrations/.__schema.sql;
	atlas schema apply \
		--to file://migrations/.__schema.sql \
		-u "postgres://portfolio:portfolio@127.0.0.1/portfolio?sslmode=disable" \
		--dev-url "docker://postgres/18/diffs";
	rm migrations/.__schema.sql;
