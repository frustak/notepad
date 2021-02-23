-- Your SQL goes here
CREATE TABLE account (
    account_id INT GENERATED ALWAYS AS IDENTITY UNIQUE PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);
