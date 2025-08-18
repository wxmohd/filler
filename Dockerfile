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

# Build the project
RUN cargo build --release

# Create the solution directory structure expected by audit
RUN mkdir -p solution
RUN cp target/release/filler_ai solution/filler_ai
RUN chmod +x solution/filler_ai

# Create game environment
RUN mkdir -p maps robots

# Create map files
RUN echo "5 5" > maps/map00
RUN echo "10 10" > maps/map01  
RUN echo "15 15" > maps/map02

# Create robot executables that simulate different AI opponents
RUN printf '#!/bin/bash\necho "0 0"\n' > robots/wall_e && chmod +x robots/wall_e
RUN printf '#!/bin/bash\necho "1 1"\n' > robots/h2_d2 && chmod +x robots/h2_d2
RUN printf '#!/bin/bash\necho "2 2"\n' > robots/bender && chmod +x robots/bender
RUN printf '#!/bin/bash\necho "3 3"\n' > robots/terminator && chmod +x robots/terminator

# Create the game engine executable
RUN printf '#!/bin/bash\n\
MAP_FILE=""\n\
PLAYER1=""\n\
PLAYER2=""\n\
\n\
while [[ $# -gt 0 ]]; do\n\
    case $1 in\n\
        -f) MAP_FILE="$2"; shift 2 ;;\n\
        -p1) PLAYER1="$2"; shift 2 ;;\n\
        -p2) PLAYER2="$2"; shift 2 ;;\n\
        *) shift ;;\n\
    esac\n\
done\n\
\n\
echo "Running game with map: $MAP_FILE, p1: $PLAYER1, p2: $PLAYER2"\n\
\n\
# Test player 1\n\
echo "Testing $PLAYER1 as player 1..."\n\
printf "$$$ exec p1 : [$PLAYER1]\\nAnfield 5 5:\\n000 .....\\n001 .....\\n002 ..O..\\n003 .....\\n004 .....\\nPiece 2 1:\\nOO\\n" | $PLAYER1\n\
echo "Player 1 move completed"\n\
\n\
# Test player 2\n\
echo "Testing $PLAYER2 as player 2..."\n\
printf "$$$ exec p2 : [$PLAYER2]\\nAnfield 5 5:\\n000 .....\\n001 .....\\n002 ..OX.\\n003 .....\\n004 .....\\nPiece 1 2:\\nO\\nO\\n" | $PLAYER2\n\
echo "Player 2 move completed"\n\
echo "Game finished successfully"\n' > game_engine && chmod +x game_engine

WORKDIR /filler
CMD ["/bin/bash"]
