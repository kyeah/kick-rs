CREATE SCHEMA IF NOT EXISTS kickstarter;

-- Only use these domains when the ORM model generator supports domain interpretation.
-- Otherwise, use _alnum and _numtext check constraints.

-- CREATE DOMAIN alnum AS text CHECK (value ~ '^[a-zA-Z0-9_-]+$');
-- CREATE DOMAIN numtext AS text CHECK (value ~ '^[0-9]+$');

CREATE TABLE IF NOT EXISTS kickstarter.users (
    user_id SERIAL NOT NULL,
    name text NOT NULL,
    date_created timestamp DEFAULT localtimestamp NOT NULL,
    PRIMARY KEY (user_id),
    CONSTRAINT user_name_uniq UNIQUE (name),
    CONSTRAINT user_name_alnum_chk CHECK (name ~ '^[a-zA-Z0-9_-]+$'),
    CONSTRAINT user_name_length_chk CHECK (char_length(name) >= 4 AND char_length(name) <= 20)
);

CREATE TABLE IF NOT EXISTS kickstarter.projects (
    project_id SERIAL NOT NULL,
    name text NOT NULL,
    goal numeric(30,2) NOT NULL,
    date_created timestamp DEFAULT localtimestamp NOT NULL,
    PRIMARY KEY (project_id),
    CONSTRAINT project_name_uniq UNIQUE (name),
    CONSTRAINT project_name_alnum_chk CHECK (name ~ '^[a-zA-Z0-9_-]+$'),
    CONSTRAINT project_name_length_chk CHECK (char_length(name) >= 4 AND char_length(name) <= 20)
);

CREATE TABLE IF NOT EXISTS kickstarter.pledges (
    user_id integer NOT NULL,
    project_id integer NOT NULL,
    amount numeric(30,2) NOT NULL,
    card text NOT NULL,
    date_created timestamp DEFAULT localtimestamp NOT NULL,
    PRIMARY KEY (user_id, project_id),
    CONSTRAINT pledge_card_numtext_chk CHECK (card ~ '^[0-9]+$'),
    CONSTRAINT pledge_card_length_chk CHECK (char_length(card) <= 19),
    CONSTRAINT pledge_project_card UNIQUE (project_id, card),
    CONSTRAINT pledge_user_fkey FOREIGN KEY ("user_id") REFERENCES kickstarter.users ("user_id") ON DELETE CASCADE,
    CONSTRAINT pledge_project_fkey FOREIGN KEY ("project_id") REFERENCES kickstarter.projects ("project_id") ON DELETE CASCADE
);

CREATE INDEX ON kickstarter.users (name);
CREATE INDEX ON kickstarter.projects (name);
CREATE INDEX ON kickstarter.pledges (user_id);
CREATE INDEX ON kickstarter.pledges (project_id);
