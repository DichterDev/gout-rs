CREATE DATABASE gout_prod;

CREATE TABLE users (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    password TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true
);
