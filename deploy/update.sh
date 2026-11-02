#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"

docker network inspect coolify >/dev/null
docker compose config --quiet
image=$(docker compose config --images)
docker image inspect "$image" >/dev/null
docker compose up -d --pull never --no-build raytracer

container_id=$(docker compose ps -q raytracer)
container_ip=$(docker inspect --format '{{with index .NetworkSettings.Networks "coolify"}}{{.IPAddress}}{{end}}' "$container_id")
if [[ -z "$container_ip" ]]; then
  echo 'Raytracer has no address on the proxy network.' >&2
  exit 1
fi
curl --fail --silent --show-error --retry 10 --retry-all-errors --retry-delay 1 \
  --max-time 3 "http://${container_ip}:8080/healthz"
printf '\nRaytracer is responding; HTTPS is served by the shared Traefik proxy.\n'
