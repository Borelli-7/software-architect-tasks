-- Placeholder initial migration for the iam service.
-- Real schema (users, role grants) will be defined here; kept non-empty so
-- `sqlx::migrate!()` finds the migrations directory at compile time.
SELECT 1;
