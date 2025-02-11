# Build Stage (with Node.js image for Elm)
FROM node:18 AS frontend-builder

# Install Elm
RUN npm install -g elm

# Copy frontend source and build
WORKDIR /frontend
COPY frontend/ .
RUN elm make src/Main.elm --output=dist/main.js

# Build Stage (with Rust)
FROM rust:1.75 AS backend-builder

# Copy backend source and build
WORKDIR /backend
COPY backend/ .
COPY --from=frontend-builder /frontend/dist/main.js  /backend/static/

RUN cargo build --release

# Runtime Stage
FROM debian:bookworm-slim

# Set working directory
WORKDIR /app

# Install OpenSSL 3 and certificates
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy Rust binary and frontend files from builder stage
COPY --from=backend-builder /backend/target/release/backend ./backend
COPY --from=backend-builder /backend/static/ ./static/

# Expose ports and run the app
EXPOSE 3001
CMD ["./backend"]