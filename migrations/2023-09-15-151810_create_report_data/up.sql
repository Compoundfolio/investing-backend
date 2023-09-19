CREATE TYPE operation_source_type AS ENUM ('exante_report', 'freedomfinance_report');
CREATE TYPE trade_side_type AS ENUM ('buy', 'sell');
CREATE TYPE custom_money AS (
    value DECIMAL,
    currecy VARCHAR(3)
);

CREATE TABLE trade_operation (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    app_user_id UUID NOT NULL REFERENCES app_user (id),
    operation_source operation_source_type NOT NULL,
    external_id VARCHAR NOT NULL,
    date_time TIMESTAMP NOT NULL,
    side trade_side_type NOT NULL,
    instrument_symbol VARCHAR NOT NULL,
    isin VARCHAR NOT NULL,
    price custom_money NOT NULL,
    quantity INTEGER,
    commission custom_money NULL,
    order_id VARCHAR NOT NULL,
    summ custom_money NOT NULL,
    metadata jsonb NOT NULL
);

CREATE TABLE transaction (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    app_user_id UUID NOT NULL REFERENCES app_user (id),
    operation_source operation_source_type NOT NULL,
    external_id VARCHAR NOT NULL,
    date_time TIMESTAMP NOT NULL,
    symbol_id VARCHAR NULL,
    amount custom_money NOT NULL,
    operation_type varchar NOT NULL,
    commission custom_money NOT NULL,
    metadata jsonb NOT NULL
);

CREATE UNIQUE INDEX ON trade_operation (operation_source, external_id);
CREATE UNIQUE INDEX ON transaction (operation_source, external_id);
