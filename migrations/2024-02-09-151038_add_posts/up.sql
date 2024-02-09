CREATE TABLE posts (
    id SERIAL PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    user_id INTEGER REFERENCES users(id),
    posting_date DATE NOT NULL DEFAULT CURRENT_DATE
);