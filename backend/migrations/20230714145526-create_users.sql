CREATE TABLE "domains" (
    "domain" TEXT NOT NULL PRIMARY KEY,
    "recognized_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "users" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "id_seq" SERIAL NOT NULL UNIQUE,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    "username" TEXT NOT NULL,
    "domain" TEXT NOT NULL,
    "public_key" TEXT NOT NULL,

    "display_name" TEXT NULL,
    "description" TEXT NULL
);
CREATE UNIQUE INDEX "users_unique_acct" ON "users" ("username", "domain");

CREATE TABLE "local_users" (
    "user_id" TEXT NOT NULL PRIMARY KEY REFERENCES "users" ("id"),
    "private_key" TEXT NOT NULL
);
