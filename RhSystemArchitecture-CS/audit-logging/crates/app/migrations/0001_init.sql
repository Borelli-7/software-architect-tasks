-- Placeholder initial migration for the audit-logging service.
-- Real schema (append-only audit_entries table) will be defined here; kept
-- non-empty so `sqlx::migrate!()` finds the migrations directory at compile time.
SELECT 1;
