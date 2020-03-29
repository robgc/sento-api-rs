# sento-api-rs

**This README is currently WIP**.

**NOTE:** This project is a port of the HTTP API made in Python and introduces breaking changes
in some routes.

# Table of contents

- [Considerations](#considerations)
- [Prerequisites](#prerequisites)
  - [Using Docker and Docker Compose](#using-docker-and-docker-compose)
  - [Running locally](#running-locally)
- [Setting up the API](#setting-up-the-api)
- [License](#license)

## Considerations

- This is v2.0 of the HTTP API, it introduces breaking changes to some routes of v1.0. Those
  breaking changes were introduced in favor of making the API more _RESTy_.
- This project does not perform database migration management as the previous did
  with Alembic.
- This project also is an Async HTTP REST API, database queries are performed asynchronously.
- The supported configuration variables may have changed, some aspects need further investigation.

## Prerequisites

### Using Docker and Docker Compose

For this type of installation you will need:

- Docker Engine 17.12.0 or higher.
- Docker Compose 1.18.0 or higher.

### Running locally

These are the software requirements of each component if you want to run it
locally:

- API
  - Rust 1.36 or later (`rustup` recommended).

In order to set up an instance you need a PostgreSQL database initialised
using the instructions available in the original
[`sento-api` repository](https://github.com/robgc/sento-api#setting-up-the-database).

## Setting up the API

Make a copy of the `config.example.toml` file found in the root directory and rename it to
`config.toml`. Then you can set the values of the different sections according to your needs.

- **With Docker**:
  1. Remember that the values set in the `config.toml` file are relative to the container.
    Normally you will not need to change the listening IP and port present
    in the `config.example.toml`, but, if you make any changes you will need to keep them
    in mind for the next step.
  2. Create a `docker-compose.override.yml` file from `docker-compose.override.exammple.yml` file,
    then you can configure the IP and port mapping between your host and the container.
    You can also override other container configurations if you need.
  3. If you built the image for the API container previously, you can run
    `docker-compose up -d sento-api`.
- **Running locally**:
  1. Run the command: `cargo run --release`.

If you have followed the previous steps you should have an
instance of the API waiting for requests.

## License

The source code of this project is licensed under the GNU Affero General
Public License v3.0.
