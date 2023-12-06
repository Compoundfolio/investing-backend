CREATE TYPE broker_type AS ENUM ('exante', 'freedomfinance');
ALTER TABLE report_upload ADD broker broker_type NOT NULL DEFAULT 'exante';
ALTER TABLE report_upload ALTER COLUMN broker DROP DEFAULT;
