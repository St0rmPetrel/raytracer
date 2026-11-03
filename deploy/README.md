# VDS deployment

The app serves static files; rendering runs in the browser. Docker Compose is
sufficient for a single VDS with an existing Traefik proxy. This setup joins its
`coolify` Docker network without publishing host ports or changing other apps.
It assumes Traefik entrypoints `http` / `https` and a `letsencrypt` TLS resolver.
Adapt these names in `compose.yaml` if your proxy uses different settings.

K3s is an option when Kubernetes is itself a project goal or several services
need a common deployment platform. It adds cluster administration and a separate
networking/ingress setup; its default Traefik uses ports 80/443, which the existing
proxy already occupies. See [K3s networking](https://docs.k3s.io/networking/networking-services).
A single-node cluster does not provide redundancy against a VDS failure.

## Build and transfer

Keep the SSH destination in your local SSH config, for example under the alias
`raytracer-vds`. Server addresses and credentials do not belong in this repository.
Build on the development computer rather than using the VDS's CPU and memory:

```sh
revision=$(git rev-parse HEAD)
image="raytracer:sha-${revision}"
docker build --platform linux/amd64 -t "$image" .
# In bash/zsh, propagate failures from every stage of the transfer.
set -o pipefail
docker save "$image" | gzip | ssh raytracer-vds 'gzip -d | docker load'
```

The Rust build stage runs on the builder's native platform; WASM is portable.
The final image targets the VDS architecture. Adjust `--platform` for your server.

## Configure and start

1. Add a DNS A record for your app domain pointing to your VDS address.
2. Copy `compose.yaml`, `.env.example` and `update.sh` to `/opt/raytracer/deploy`.
3. Copy `.env.example` to `.env`. Set `RAYTRACER_IMAGE` to the transferred tag and
   `RAYTRACER_DOMAIN` to your app domain. Keep this environment file on the server.
4. Run `./update.sh`. Traefik's existing `letsencrypt` resolver issues the TLS
   certificate once DNS resolves to the VDS.
5. Verify `https://<your-app-domain>/healthz`, the app and WASM loading.

No bot token or backend is required. To launch inside Telegram, create a bot with
BotFather and set its Main Mini App URL to `https://<your-app-domain>/`.

## Update or roll back

Transfer the desired commit-tagged image, change `RAYTRACER_IMAGE` in the server's
`.env`, then run `./update.sh`. Keep the prior image for rollback. Do not rebuild
or restart the shared proxy or unrelated applications.

Useful commands, run from `/opt/raytracer/deploy`:

```sh
docker compose ps
docker compose logs --tail=100 raytracer
```
