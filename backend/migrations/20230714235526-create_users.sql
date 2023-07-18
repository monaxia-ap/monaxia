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
-- first column is domain, which is useful for ranged index search with domain.
CREATE UNIQUE INDEX "users_unique_acct" ON "users" ("domain", "username");

CREATE TABLE "local_users" (
    "user_id" TEXT NOT NULL PRIMARY KEY REFERENCES "users" ("id"),
    "private_key" TEXT NOT NULL
);
