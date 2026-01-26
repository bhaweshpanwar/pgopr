# pgmoneta: Backup management for pgopr

pgmoneta is the backup component for the pgopr operator, providing high-performance backup and restore capabilities for PostgreSQL.

## Features

- Full base backups
- Rocky Linux 10 based container
- Integration with pgopr primary instances
- Prometheus metrics support

## Requirements

- pgopr operator installed
- A running PostgreSQL primary (provisioned via pgopr)

## Usage

### Provisioning

To deploy the pgmoneta backup service:

```bash
pgopr provision pgmoneta
