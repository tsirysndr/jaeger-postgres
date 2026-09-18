# jaeger-postgres

PostgreSQL remote trace storage for Jaeger v2, implemented in Rust with Tonic,
SQLx, and SeaQuery. One gRPC listener provides:

- OTLP `TraceService/Export` for writes
- Jaeger `jaeger.storage.v2.TraceReader` for trace queries
- Jaeger `jaeger.storage.v2.DependencyReader` for the dependency graph

The original OTLP span, resource, scope, events, links, status, and schema URLs
are stored as protobuf. Indexed columns and JSONB attribute copies support
Jaeger's service, operation, time, duration, and attribute searches.

## Run

`DATABASE_URL` is required. The schema and indexes are created automatically at
startup.

```sh
export DATABASE_URL='postgres://postgres:postgres@localhost:5432/jaeger'
cargo build --release
./target/release/jaeger-postgres
```

After connecting to PostgreSQL and applying the schema, startup prints an ASCII
banner with the configured gRPC address and the services sharing its port.

Optional environment variables:

| Variable | Default | Meaning |
| --- | --- | --- |
| `LISTEN_ADDR` | `0.0.0.0:17271` | Combined OTLP and Jaeger gRPC address |
| `DATABASE_MAX_CONNECTIONS` | `20` | SQLx pool size |
| `MAX_SEARCH_DEPTH` | `1000` | Upper bound for a Jaeger trace search |
| `RUST_LOG` | `jaeger_postgres=info` | Log filter |

Every gRPC request emits a start event, a completion event with busy/idle
timings, and an error event when the RPC fails. Request payloads are not logged.

Point the Jaeger v2 remote-storage backend at port `17271` with insecure TLS for
a local deployment. The backend implements the standard OTLP writer on that
same endpoint, as required by the Jaeger storage v2 contract.

## Test

```sh
cargo test
cargo build --release
```

The migration is also available at `migrations/0001_create_spans.sql` for
operators that manage DDL separately.

## Install a release

The installer detects macOS ARM64, Linux x86_64, or Linux ARM64 and verifies the
download against the release checksums:

```sh
curl --proto '=https' --tlsv1.2 -LsSf \
  https://raw.githubusercontent.com/tsirysndr/jaeger-postgres/main/install.sh | sh
```

A root Linux installation also places the unit at
`/etc/systemd/system/jaeger-postgres.service` and creates a private environment
file. Install system-wide, configure, and start it with:

```sh
curl --proto '=https' --tlsv1.2 -LsSf \
  https://raw.githubusercontent.com/tsirysndr/jaeger-postgres/main/install.sh | sudo sh
sudo editor /etc/jaeger-postgres/env
sudo systemctl enable --now jaeger-postgres
sudo systemctl status jaeger-postgres
```

The installer intentionally does not start the service until `DATABASE_URL` is
configured. Set `JAEGER_POSTGRES_VERSION=v0.1.0` to install a particular tag,
`INSTALL_DIR` to change the binary destination, or `INSTALL_SYSTEMD=0` to skip
the unit.

Tags matching `v*` trigger release builds for Darwin ARM64, Linux x86_64, and
Linux ARM64. The workflow publishes all three archives and `SHA256SUMS` to the
corresponding GitHub release.
