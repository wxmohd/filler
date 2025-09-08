FROM rust:1.63-buster

# Copy maps and robots from docker_image
COPY docker_image/maps/ /filler/maps/
COPY docker_image/linux_robots/ /filler/linux_robots/
COPY docker_image/m1_robots/ /filler/m1_robots/
COPY docker_image/linux_game_engine /filler/
COPY docker_image/m1_game_engine /filler/

# Copy solution files from root
COPY solution/ /filler/solution/

# Create solution directory
RUN mkdir -p /filler/solution

# Set working directory
WORKDIR /filler

# Build the Rust project from modular solution files
RUN cd solution && rustc --edition 2021 -O filler_ai.rs -o filler_ai

# Set executable permissions
RUN chmod +x solution/filler_ai

# Make executables runnable
RUN chmod +x /filler/linux_game_engine /filler/m1_game_engine /filler/linux_robots/* /filler/m1_robots/*

WORKDIR /filler/

ENTRYPOINT ["/bin/bash"]
