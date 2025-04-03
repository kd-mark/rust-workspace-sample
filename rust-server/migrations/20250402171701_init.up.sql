-- Add migration script here

CREATE TABLE files (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    size BIGINT NOT NULL,
    file_ref VARCHAR(255) NOT NULL
);


CREATE TYPE status_enum AS ENUM ('compressing', 'passed', 'failed');
CREATE TABLE compressed_files (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    file_id UUID REFERENCES files(id),
    status status_enum DEFAULT 'compressing' NOT NULL,
    alg VARCHAR(255) NOT NULL,
    level INTEGER NOT NULL,
    file_ref VARCHAR(255) NOT NULL
);
