-- Your SQL goes here
CREATE TABLE notepad (
    notepad_id INT GENERATED ALWAYS AS IDENTITY UNIQUE PRIMARY KEY,
    note TEXT NOT NULL,
    account_id INT NOT NULL UNIQUE,
    CONSTRAINT fk_account
        FOREIGN KEY(account_id)
            REFERENCES account(account_id)
            ON DELETE CASCADE
);
