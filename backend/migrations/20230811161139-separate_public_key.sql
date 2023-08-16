ALTER TABLE "users"
    ADD COLUMN "public_key_id" TEXT NOT NULL DEFAULT '';

CREATE INDEX "users_key_id" ON "users" ("public_key_id");
