# -> Stage 1: Init cargo-chef image
FROM clux/muslrust:latest as chef
# Install cargo-chef
RUN cargo install cargo-chef 
# Work dir
WORKDIR /app

# -> Stage 1: Generate recipe file
FROM chef as planner
# Files
COPY . .
# Build the app
RUN cargo chef prepare --recipe-path recipe.json

# -> Stage 2: Build dependencies
FROM chef as cacher
# Files
COPY --from=planner /app/recipe.json recipe.json
# Build the app
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

# -> Stage 2: Build app
FROM cacher as builder
# Files
COPY . .
# Build the app
RUN cargo build --release --target=x86_64-unknown-linux-musl

# ---------------- Stage 3: Deploy app ----------------
# Use small distribution for running the app
FROM scratch
# Work dir
WORKDIR /app
# Copy app from builder
# TODO: Change axum-api-template to the name of the project
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/axum-api-template .
# Start the app
# TODO: Change axum-api-template to the name of the project
CMD ["./axum-api-template"]

# Build with docker build -t mouradost/axum-api-template .
# Save image as .tar with docker save -o ./axum-api-template.tar mouradost/axum-api-template
# Load image from .tar with docker load -i ./axum-api-template.tar mouradost/axum-api-template
# Check all docker images with docker image ls
# Remove the docker image with docker image rm mouradost/axum-api-template
# Run the docker image with docker run -t axum-api-template mouradost/axum-api-template
# Check the docker containers with docker ps -a (or docker ps to show only the running containers)
