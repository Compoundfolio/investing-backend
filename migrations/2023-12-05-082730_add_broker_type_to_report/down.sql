-- 8.
ALTER TABLE "fiscal_transaction" RENAME TO "transaction";
-- 6.
ALTER TYPE custom_money RENAME ATTRIBUTE currency TO currecy;
-- 5.
ALTER TABLE report_upload
ALTER COLUMN created_at DROP DEFAULT;
-- 3.
ALTER TABLE transaction DROP COLUMN report_upload_id;
-- 2.
ALTER TABLE report_upload DROP COLUMN broker;
DROP TYPE broker_type;

