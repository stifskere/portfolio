set dotenv-load

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
