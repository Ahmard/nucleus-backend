# ---------------------------------------------------
# 1 - Build Stage
#
# Use official rust image to for application build
# ---------------------------------------------------
FROM rust:1.69 as planner
RUN cargo install cargo-chef

# Setup working directory
WORKDIR /usr/src/nucleus-backend
COPY . .
COPY .env.example .env

# Prepare a build plan ("recipe")
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.69 as build
RUN cargo install cargo-chef

# Install dependency (Required by diesel)
RUN apt-get update && apt-get install libpq5 -y

# Copy the build plan from the previous Docker stage
COPY --from=planner /usr/src/nucleus-backend/recipe.json recipe.json

# Build dependencies - this layer is cached as long as `recipe.json`
# doesn't change.
RUN cargo chef cook --recipe-path recipe.json

# Build the whole project
COPY . .
COPY .env.example .env

# Install dependency (Required by diesel)
RUN apt-get update && apt-get install libpq5 -y

# Build application
RUN cargo install --path .

# ---------------------------------------------------
# 2 - Deploy Stage
#
# Use a distroless image for minimal container size
# - Copy `libpq` dependencies into the image (Required by diesel)
# - Copy application files into the image
# ---------------------------------------------------
FROM gcr.io/distroless/cc-debian11

# Set the architecture argument (arm64, i.e. aarch64 as default)
# For amd64, i.e. x86_64, you can append a flag when invoking the build `... --build-arg "ARCH=x86_64"`
ARG ARCH=aarch64

# Application files
COPY --from=build /usr/local/cargo/bin/nucleus /usr/local/bin/nucleus
#COPY --from=build /usr/src/nucleus-backend/.env /.env

CMD ["/usr/local/bin/nucleus"]