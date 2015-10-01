CREATE SCHEMA IF NOT EXISTS kickstarter;

CREATE TABLE IF NOT EXISTS kickstarter.user (
    user_id SERIAL NOT NULL,
    name text NOT NULL,
    date_created timestamp DEFAULT localtimestamp NOT NULL,
    PRIMARY KEY (user_id),
    CONSTRAINT user_name_uniq UNIQUE (name)
);

CREATE TABLE IF NOT EXISTS kickstarter.project (
    project_id SERIAL NOT NULL,
    name text NOT NULL,
    goal numeric NOT NULL,
    date_created timestamp DEFAULT localtimestamp NOT NULL,
    PRIMARY KEY (project_id),
    CONSTRAINT project_name_uniq UNIQUE (name)
);

CREATE TABLE IF NOT EXISTS kickstarter.backing (
    user_id integer NOT NULL,
    project_id integer NOT NULL,
    amount numeric NOT NULL,
    card text NOT NULL,
    date_created timestamp DEFAULT localtimestamp NOT NULL,
    PRIMARY KEY (user_id, project_id),
    CONSTRAINT backing_project_card UNIQUE (project_id, card),
    CONSTRAINT backing_user_fkey FOREIGN KEY ("user_id") REFERENCES kickstarter.user ("user_id") ON DELETE CASCADE,
    CONSTRAINT backing_project_fkey FOREIGN KEY ("project_id") REFERENCES kickstarter.project ("project_id") ON DELETE CASCADE
);

CREATE INDEX ON kickstarter.user (name);
CREATE INDEX ON kickstarter.project (name);
CREATE INDEX ON kickstarter.backing (user_id);
CREATE INDEX ON kickstarter.backing (project_id);
