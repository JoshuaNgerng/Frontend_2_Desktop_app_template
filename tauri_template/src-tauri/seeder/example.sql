-- migration: 001_seed_users.sql

INSERT INTO users (email, name)
VALUES ('admin@mail.com', 'Admin')
ON CONFLICT (email) DO NOTHING;