CREATE TABLE users (
  user_id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  date_created TIMESTAMP DEFAULT LOCALTIMESTAMP NOT NULL,
  CONSTRAINT user_name_uniq UNIQUE (name),
  CONSTRAINT user_name_alnum_chk CHECK (name ~ '[a-zA-Z0-9_-]+$'),
  CONSTRAINT user_name_length_chk CHECK (char_length(name) >= 4 AND char_length(name) <= 20)
)
