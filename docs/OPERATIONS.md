# Operations Guide

## Runtime Profile

- Project: `rust-rest-api`
- Primary stack: Rust
- File count at enhancement time: 12

## Local Development Checklist

- Build with `cargo build --release`.
- Run `cargo test` and `cargo clippy`.
- Keep runtime configuration in sync with `.env.example`.

## Release Checklist

- Review `.env.example` and confirm required environment variables.
- Run tests and static validation before publishing.
- Review database migrations and seed data changes.
- Confirm health checks and CI workflows still reflect the runtime architecture.
- Update README and architecture notes if behavior changed.

## Troubleshooting

- Start with the generated CI workflow to see the intended build and test flow.
- Check environment variables first when authentication or connectivity fails.
- Validate database connectivity before debugging application-layer failures.
- Keep logs structured and avoid hiding infrastructure errors behind generic handlers.
