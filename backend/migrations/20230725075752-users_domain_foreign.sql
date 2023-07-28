ALTER TABLE "users"
    ADD FOREIGN KEY ("domain") REFERENCES "domains" ("domain");
