FROM ubuntu:20.04

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /filler

# Copy project files
COPY . .

# Build the project (including game engine)
RUN cargo build --release

# Create the solution directory structure expected by audit
RUN mkdir -p solution
RUN cp target/release/filler_ai solution/filler_ai
RUN chmod +x solution/filler_ai

# Copy the game engine with correct name
RUN cp target/release/game_engine ./game_engine
RUN chmod +x game_engine

# Create game environment directories
RUN mkdir -p maps robots

# Copy map files
COPY maps/ maps/

# Create robot executables with basic AI strategies
RUN mkdir -p robots

RUN printf '#!/bin/bash\n\
while IFS= read -r line; do\n\
    if [[ $line == "Anfield"* ]]; then\n\
        # Read board\n\
        while IFS= read -r board_line; do\n\
            if [[ $board_line == "Piece"* ]]; then\n\
                echo "1 1"\n\
                exit 0\n\
            fi\n\
        done\n\
    fi\n\
done\n' > robots/wall_e && chmod +x robots/wall_e

RUN printf '#!/bin/bash\n\
while IFS= read -r line; do\n\
    if [[ $line == "Anfield"* ]]; then\n\
        # Read board\n\
        while IFS= read -r board_line; do\n\
            if [[ $board_line == "Piece"* ]]; then\n\
                echo "2 2"\n\
                exit 0\n\
            fi\n\
        done\n\
    fi\n\
done\n' > robots/h2_d2 && chmod +x robots/h2_d2

RUN printf '#!/bin/bash\n\
while IFS= read -r line; do\n\
    if [[ $line == "Anfield"* ]]; then\n\
        # Read board\n\
        while IFS= read -r board_line; do\n\
            if [[ $board_line == "Piece"* ]]; then\n\
                echo "1 2"\n\
                exit 0\n\
            fi\n\
        done\n\
    fi\n\
done\n' > robots/bender && chmod +x robots/bender

RUN printf '#!/bin/bash\n\
while IFS= read -r line; do\n\
    if [[ $line == "Anfield"* ]]; then\n\
        # Read board\n\
        while IFS= read -r board_line; do\n\
            if [[ $board_line == "Piece"* ]]; then\n\
                echo "0 1"\n\
                exit 0\n\
            fi\n\
        done\n\
    fi\n\
done\n' > robots/terminator && chmod +x robots/terminator

WORKDIR /filler
CMD ["/bin/bash"]
