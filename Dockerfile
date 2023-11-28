# FROM rust:1.70.0
#
# EXPOSE 3000
#
# COPY . /app
# WORKDIR /app
#
# RUN cargo build
#
# CMD ["cargo", "run"]

FROM rust:1.70.0 as builder

WORKDIR /usr/app
RUN USER=root cargo new --bin school_calendar
WORKDIR /usr/app/school_calendar

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/school_calendar*
RUN cargo build --release

# -----
FROM debian:bullseye-slim

RUN apt-get update 
RUN apt-get install -y curl && rm -rf /var/lib/apt/lists/*

EXPOSE 3000

WORKDIR /usr/app
COPY --from=builder /usr/app/school_calendar/target/release/school_calendar /usr/app/school_calendar

CMD ["/usr/app/school_calendar"]
