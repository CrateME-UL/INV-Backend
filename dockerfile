FROM rust:1.80.1-slim-bookworm AS build

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl3 \
    libssl-dev \
    clang \
    && rm -rf /var/lib/apt/lists/*

RUN rustup update stable
RUN cargo install sqlx-cli

WORKDIR /app
COPY ./build.sh /app
COPY ./scripts /app/scripts
COPY ./server /app/server

ENV APP_DIR=/app
ENV SQLX_OFFLINE=true
RUN chmod +x /app/build.sh
RUN /app/build.sh

FROM rust:1.80.1-slim-bookworm
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl3 \
    libssl-dev \
    postgresql-client \
    netcat-traditional \
    clang \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/server/target/release/app /app/server/target/release/app
COPY --from=build /app/scripts/excel_to_sql/target/debug/excel_to_sql /app/scripts/excel_to_sql/target/debug/excel_to_sql
COPY --from=build /app/scripts/db_script.sql /app/scripts/db_script.sql
COPY --from=build /app/scripts/excel_to_sql/map_inventaire.xlsx /app/scripts/excel_to_sql/target/debug/map_inventaire.xlsx
COPY ./connect_db.sh /app
COPY ./populate.sh /app
COPY ./entrypoint.sh /app

ENV POSTGRES_USER=some-postgres
ENV POSTGRES_DB=some-postgres
ENV POSTGRES_PASSWORD=mysecretpassword
ENV DATABASE_PORT=5432
ENV APP_DIR=/app
ENV DATABASE_URL=postgres://some-postgres:mysecretpassword@inv-db:5432/some-postgres
ENV DATABASE_HOST=inv-db
ENV EXCEL_PATH=/app/scripts/excel_to_sql/target/debug/map_inventaire.xlsx

RUN chmod +x /app/connect_db.sh
RUN chmod +x /app/populate.sh
RUN chmod +x /app/entrypoint.sh
ENTRYPOINT /app/entrypoint.sh