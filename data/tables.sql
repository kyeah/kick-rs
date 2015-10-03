CREATE SCHEMA IF NOT EXISTS kickstarter;

-- Only use these domains when the ORM model generator supports domain interpretation.
-- Otherwise, use _alnum and _numtext check constraints.

-- CREATE DOMAIN alnum AS text CHECK (value ~ '^[a-zA-Z0-9_-]+$');
-- CREATE DOMAIN numtext AS text CHECK (value ~ '^[0-9]+$');

-- TODO: Add numeric(30,2), decimal(30,2) or money type support to rust-postgres to replace DOUBLE.

-- User:    indexes on user_id and name.
-- Project: indexes on project_id and name.
-- Pledge:  indexes on user_id, project_id, and card.

CREATE TABLE IF NOT EXISTS kickstarter.user (
    user_id SERIAL NOT NULL,
    name text NOT NULL,
    date_created timestamp DEFAULT localtimestamp NOT NULL,
    PRIMARY KEY (user_id),
    CONSTRAINT user_name_uniq UNIQUE (name),
    CONSTRAINT user_name_alnum_chk CHECK (name ~ '^[a-zA-Z0-9_-]+$'),
    CONSTRAINT user_name_length_chk CHECK (char_length(name) >= 4 AND char_length(name) <= 20)
);

CREATE TABLE IF NOT EXISTS kickstarter.project (
    project_id SERIAL NOT NULL,
    name text NOT NULL,
    goal double precision NOT NULL,
    date_created timestamp DEFAULT localtimestamp NOT NULL,
    PRIMARY KEY (project_id),
    CONSTRAINT project_name_uniq UNIQUE (name),
    CONSTRAINT project_name_alnum_chk CHECK (name ~ '^[a-zA-Z0-9_-]+$'),
    CONSTRAINT project_name_length_chk CHECK (char_length(name) >= 4 AND char_length(name) <= 20)
);

CREATE TABLE IF NOT EXISTS kickstarter.pledge (
    user_id integer NOT NULL,
    project_id integer NOT NULL,
    amount double precision NOT NULL,
    card text NOT NULL,
    date_created timestamp DEFAULT localtimestamp NOT NULL,
    PRIMARY KEY (user_id, project_id),
    CONSTRAINT pledge_card_numtext_chk CHECK (card ~ '^[0-9]+$'),
    CONSTRAINT pledge_card_length_chk CHECK (char_length(card) <= 19),
    CONSTRAINT pledge_project_card UNIQUE (project_id, card),
    CONSTRAINT pledge_user_fkey FOREIGN KEY ("user_id") REFERENCES kickstarter.user ("user_id") ON DELETE CASCADE,
    CONSTRAINT pledge_project_fkey FOREIGN KEY ("project_id") REFERENCES kickstarter.project ("project_id") ON DELETE CASCADE
);

CREATE OR REPLACE FUNCTION upsert_user(_name text) RETURNS integer AS $$
DECLARE
    return_id integer;
BEGIN
    with s as (SELECT user_id FROM kickstarter.user WHERE name = _name),
         i as (INSERT INTO kickstarter.user (name)
               SELECT _name
               WHERE NOT EXISTS (SELECT 1 FROM s)
               RETURNING user_id)

    SELECT user_id FROM i
    UNION ALL 
    SELECT user_id FROM s
    INTO return_id;

    return return_id;
END;
$$ LANGUAGE plpgsql;
