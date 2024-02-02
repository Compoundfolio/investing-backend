-- 2.
ALTER TABLE report_upload DROP COLUMN broker;
DROP TYPE broker_type;
-- 3.
ALTER TABLE transaction DROP COLUMN report_upload_id;
-- 5.
ALTER TABLE report_upload
ALTER COLUMN created_at DROP DEFAULT;
