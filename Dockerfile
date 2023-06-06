# Build
FROM rust:1.70 as planner
RUN cargo install cargo-chef

# Set work directory
WORKDIR /usr/src/nucleus-backend
COPY . .

# Prepare a build plan ("recipe")
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.70 as build
RUN cargo install cargo-chef

# Install postgres library
RUN apt-get update && apt-get install libpq5 -y

# Copy the build plan from the previous Docker stage
COPY --from=planner /usr/src/nucleus-backend/recipe.json recipe.json

# Build dependencies - this layer is cached as long as `recipe.json`
# doesn't change.
RUN cargo chef cook --recipe-path recipe.json

# Build the whole project
COPY . .

# Setup working directory
WORKDIR /nucleus

# Build application
RUN cargo build --release

# BUILD
FROM rust:1.70 AS runtime

# Install dependency (Required by diesel)
RUN apt-get update && apt-get install libpq5 -y

# Install Diesel CLI
RUN cargo install diesel_cli --no-default-features --features postgres

COPY .env .env
COPY static static
COPY templates templates
COPY migrations migrations

# Copy our built binary
COPY --from=build /target/release/nucleus /usr/local/bin/nucleus

CMD ["nucleus"]