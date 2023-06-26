CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE app_user (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR UNIQUE NOT NULL
);

CREATE TYPE login_method_type_type AS ENUM ('google_oauth', 'password');

CREATE TABLE app_user_login_method (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    app_user_id UUID NOT NULL REFERENCES app_user (id),
    login_method_type login_method_type_type NOT NULL,
    subject_id VARCHAR,
    password_hash VARCHAR,
    CONSTRAINT password_hash_for_password_method
        CHECK (
            login_method_type = 'password' AND subject_id IS NULL
            OR
            login_method_type = 'google_oauth' AND subject_id IS NOT NULL AND password_hash IS NULL 
        )
);
