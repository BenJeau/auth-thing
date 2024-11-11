# START -- Rust backend builder 
FROM rust:1.82-alpine3.20 AS rust-builder-chef
RUN apk add --no-cache musl-dev gcc libc-dev
RUN cargo install --version 0.1.68 cargo-chef
WORKDIR /app

FROM rust-builder-chef AS rust-planner
COPY /backend .
RUN cargo chef prepare --recipe-path recipe.json --bin server

FROM rust-builder-chef AS rust-builder
COPY --from=rust-planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --bin server
COPY /backend .

ARG SERVER_VERSION
ARG COMMIT_GIT_SHA

ENV SERVER_VERSION=${SERVER_VERSION}
ENV COMMIT_GIT_SHA=${COMMIT_GIT_SHA}

RUN SQLX_OFFLINE=true cargo build --release --bin server
RUN touch /db.sqlite
# END -- Rust backend builder

# START -- React frontend builder
FROM node:22.11.0-alpine3.20 AS react-base

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
COPY /frontend /app
WORKDIR /app

FROM react-base AS react-builder
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile

ARG VITE_VERSION
ARG VITE_COMMIT_SHA

ENV VITE_VERSION=${VITE_VERSION}
ENV VITE_COMMIT_SHA=${VITE_COMMIT_SHA}

RUN pnpm run build
# END -- Rust backend builder

FROM scratch AS final
COPY --from=rust-builder /app/target/release/server /
COPY --from=rust-builder /db.sqlite /
COPY --from=react-builder /app/dist /dist

EXPOSE 3456

CMD [ "/server" ]