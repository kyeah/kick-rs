CREATE TABLE projects (
  project_id SERIAL NOT NULL,
  name text NOT NULL,
  goal double precision NOT NULL,
  date_created timestamp DEFAULT localtimestamp NOT NULL,
  PRIMARY KEY (project_id),
  CONSTRAINT project_name_uniq UNIQUE (name),
  CONSTRAINT project_name_alnum_chk CHECK (name ~ '^[a-zA-Z0-9_-]+$'),
  CONSTRAINT project_name_length_chk CHECK (char_length(name) >= 4 AND char_length(name) <= 20)
)
