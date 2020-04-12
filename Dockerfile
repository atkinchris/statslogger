FROM rust:1-stretch

VOLUME [ "/app/target" ]

WORKDIR /app
COPY . .

ENTRYPOINT [ "cargo", "run", "--" ]
