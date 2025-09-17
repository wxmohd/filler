FROM rust:latest

# Install required tools
RUN apt-get update && \
    apt-get install -y dos2unix && \
    rm -rf /var/lib/apt/lists/*

# Create solution directory
RUN mkdir -p /filler/solution

# Set working directory
WORKDIR /filler

# Copy project files
COPY Cargo.toml /filler/
COPY solution/ /filler/solution/

# Build the project
RUN cargo build --release --bin filler_ai

# Copy the built binary
RUN cp /filler/target/release/filler_ai /filler/solution/filler_ai

# Copy game files
COPY docker_image/maps/ /filler/maps/
COPY docker_image/linux_robots/ /filler/linux_robots/
COPY docker_image/m1_robots/ /filler/m1_robots/
COPY docker_image/linux_game_engine /filler/
COPY docker_image/m1_game_engine /filler/

# Fix map files
RUN for map in /filler/maps/*; do \
    # Convert Windows line endings to Unix
    dos2unix "$map" && \
    # Add dimensions as first line (get dimensions from filename or hardcode)
    if [[ "$(basename "$map")" == "map00" ]]; then \
        sed -i '1i20 15' "$map"; \
    elif [[ "$(basename "$map")" == "map01" ]]; then \
        sed -i '1i40 24' "$map"; \
    elif [[ "$(basename "$map")" == "map02" ]]; then \
        sed -i '1i100 100' "$map"; \
    fi \
    done

# Set permissions
RUN chmod +x /filler/solution/filler_ai && \
    chmod +x /filler/linux_game_engine /filler/m1_game_engine && \
    chmod +x /filler/linux_robots/* /filler/m1_robots/*

WORKDIR /filler
ENTRYPOINT ["/bin/bash"]