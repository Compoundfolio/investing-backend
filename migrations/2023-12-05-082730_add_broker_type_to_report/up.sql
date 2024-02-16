-- 1. Add 'manually uploaded' operation source
ALTER TYPE operation_source_type ADD VALUE IF NOT EXISTS 'manual';

-- 2. Add 'broker' into the report upload
CREATE TYPE broker_type AS ENUM ('exante', 'freedomfinance');
ALTER TABLE report_upload 
    ADD broker broker_type NOT NULL DEFAULT 'exante';
ALTER TABLE report_upload 
    ALTER COLUMN broker DROP DEFAULT;
ALTER TABLE transaction
    ADD broker broker_type;
ALTER TABLE trade_operation
    ADD broker broker_type;
-- 3. Add upload id into transaction
ALTER TABLE transaction
    ADD report_upload_id UUID NULL REFERENCES report_upload (id);

-- 5. For report uploads - make created at a default field
ALTER TABLE report_upload
ALTER COLUMN created_at SET DEFAULT current_timestamp;

-- 6. Fix money max length
ALTER TYPE custom_money RENAME ATTRIBUTE currecy TO currency;
CREATE TYPE custom_money_temp AS (
    amount DECIMAL,
    currency TEXT
);
ALTER TABLE transaction
    ALTER COLUMN amount TYPE custom_money_temp USING '(0,"")'::custom_money_temp;
ALTER TABLE transaction
    ALTER COLUMN commission TYPE custom_money_temp USING '(0,"")'::custom_money_temp;
ALTER TABLE trade_operation
    ALTER COLUMN price TYPE custom_money_temp USING '(0,"")'::custom_money_temp;
ALTER TABLE trade_operation
    ALTER COLUMN commission TYPE custom_money_temp USING '(0,"")'::custom_money_temp;
ALTER TABLE trade_operation
    ALTER COLUMN summ TYPE custom_money_temp USING '(0,"")'::custom_money_temp;
DROP TYPE custom_money;
ALTER TYPE custom_money_temp RENAME TO custom_money;

-- 7. Make some columns optional
ALTER TABLE "transaction" 
    ALTER COLUMN commission DROP NOT NULL;
ALTER TABLE "transaction"
    ALTER COLUMN external_id DROP NOT NULL;
ALTER TABLE "trade_operation"
    ALTER COLUMN external_id DROP NOT NULL;
ALTER TABLE "trade_operation"
    ALTER COLUMN quantity SET NOT NULL;

-- 8. Rename transaction table
ALTER TABLE "transaction" RENAME TO fiscal_transaction;

-- 9. Cleanup of useless brokerage data
DELETE FROM fiscal_transaction
WHERE LOWER(operation_type)='trade'
