set dotenv-load

just := `command -v just`

# just dev
#
# Starts the development environment.
#
# Triggers `docker compose` to build and start the dev environment,
# the definitions are at `docker/dev.docker-compose.yml`.
@dev:
	#!/bin/bash
	set -e;

	if
		[[ -f /.dockerenv ]] ||
		grep -qE '(docker|containerd|kubepods)' /proc/1/cgroup 2>/dev/null;
	then
		# This watches the migrations directory and re-runs migrations
		# when edited. Only if $AUTO_MIGRATE is true.
		${PF_AUTO_MIGRATE:=false}
		if [ "$PF_AUTO_MIGRATE" = true ]; then
			watchexec -w ./migrations -r -- {{just}} migrate &
		fi

		# This watches the backend and restarts the service when edited.
		watchexec -w ./core/backend -r -- cargo run --bin backend &
		# Trunk by itself already acts as a file watcher, see `Trunk.toml`
		trunk serve --config Trunk.toml 2>&1 &
		wait;
	else
		if !command -v docker >/dev/null 2>&1; then
			echo "Please, install a linux compatible version of docker before continuing.";
			exit 1;
		fi

		docker compose -f ./infrastructure/dev/docker/docker-compose.yml up --no-deps --build;
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

		docker build --build-arg BUILD_STAGE=true -t "$image_name:$version" -f ./infrastructure/prod/docker/dockerfile .;
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

	# This function is meant to (for now) only get the version
	# of all the crates in this project.
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


# Migrates all database migration files into the database specified
# by the `$PF_DATABASE_URL` environment variable.
#
# This process first flattens the migrations directory structure
# into a single-level directory, then applies the migrations.
#
# The destination directory for the flattened migrations must be
# provided via the `$PF_MIGRATIONS_FLATDIR` environment variable.
@migrate directory="migrations" url="":
	#!/bin/bash
	set -e;

	# Check whether we can obtain a database URL from somewhere.
	DATABASE_URL="{{url}}"
	DATABASE_URL="${DATABASE_URL:-$PF_DATABASE_URL}"
	if [[ -z "$DATABASE_URL" ]]; then
		echo "Missing PF_DATABASE_URL or the url parameter.";
		exit 1;
	fi

	# Check if there is a defined path where to flatten the migrations
	# directory.
	if [[ -z "$PF_MIGRATIONS_FLATDIR" ]]; then
		echo "Missing PF_MIGRATIONS_FLATDIR variable, cannot migrate.";
		exit 1;
	fi

	# Re-create the flattened migrations directory.
	rm -rf --preserve-root $PF_MIGRATIONS_FLATDIR || true;
	mkdir -p $PF_MIGRATIONS_FLATDIR;

	# Copy all the files from {{directory}} inside $MIGRATIONS_FLATDIR
	# deterministically.
	find "{{directory}}" -type f -name "*.hcl" -print0 |
	while IFS= read -r -d '' src; do
		rel="${src#"{{directory}}/"}"
		flat="${rel//\//_}"
		hash=$(printf "%s" "$rel" | sha1sum | cut -c1-8)

		cp "$src" "$PF_MIGRATIONS_FLATDIR/${hash}__${flat}"
	done

	# Apply the migrations with the anterior processed constraints.
	atlas schema apply \
		--to "file://$PF_MIGRATIONS_FLATDIR" \
		-u "$DATABASE_URL?sslmode=disable" \
		--auto-approve;
