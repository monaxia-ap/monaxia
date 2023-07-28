# Backend binary---------------------------------------------------------------

FROM rust:1.71-slim-bookworm AS backend-builder
WORKDIR /usr/src/monaxia
COPY ./backend .
RUN cargo install --path ./monaxia

# Frontend assets -------------------------------------------------------------

FROM debian:bookworm-slim AS frontend-builder
WORKDIR /usr/src/monaxia
COPY ./frontend .

# Running image ---------------------------------------------------------------

FROM debian:bookworm-slim
LABEL maintainer="kb10uy"
ENV TZ="Asia/Tokyo"
ENV PATH="/monaxia:/usr/local/bin:/usr/bin:/bin:/sbin"

RUN apt-get update \
    && apt-get upgrade \
    && apt-get install -y --no-install-recommends \
        tzdata curl ffmpeg \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists \
    && cp -f "/usr/share/zoneinfo/${TZ}" "/etc/localtime"

RUN groupadd -g 991 monaxia && useradd -l -u 991 -g 991 -m -d /monaxia monaxia
USER monaxia
WORKDIR /monaxia

COPY --from=backend-builder /usr/local/cargo/bin/monaxia /monaxia/monaxia
COPY --from=backend-builder /usr/src/monaxia/migrations /monaxia/migrations
COPY --from=frontend-builder /usr/src/monaxia/.dockerignore /monaxia/.dockerignore

CMD ["/monaxia/monaxia", "serve"]
