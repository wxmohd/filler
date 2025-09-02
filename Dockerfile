FROM rust:1.70-buster

# Copy our Rust project source first
COPY ./src /filler/src
COPY ./Cargo.toml /filler/Cargo.toml
# Don't copy Cargo.lock to avoid version conflicts - let cargo generate it

WORKDIR /filler

# Build our AI
RUN cargo build --release

# Create solution directory and copy our AI
RUN mkdir -p /filler/solution
RUN cp target/release/filler_ai solution/filler_ai
RUN chmod +x solution/filler_ai

# Now copy the docker_image contents (maps, robots, game engines)
COPY ./docker_image/maps /filler/maps
COPY ./docker_image/linux_robots /filler/linux_robots
COPY ./docker_image/m1_robots /filler/m1_robots
COPY ./docker_image/linux_game_engine /filler/linux_game_engine
COPY ./docker_image/m1_game_engine /filler/m1_game_engine

# Make game engines and robots executable
RUN chmod +x linux_game_engine m1_game_engine linux_robots/* m1_robots/*

ENTRYPOINT ["/bin/bash"]
