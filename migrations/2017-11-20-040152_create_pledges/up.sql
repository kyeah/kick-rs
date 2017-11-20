CREATE TABLE pledges (
  user_id integer NOT NULL,
  project_id integer NOT NULL,
  amount double precision NOT NULL,
  card text NOT NULL,
  date_created timestamp DEFAULT localtimestamp NOT NULL,
  PRIMARY KEY (user_id, project_id),
  CONSTRAINT pledge_card_numtext_chk CHECK (card ~ '^[0-9]+$'),
  CONSTRAINT pledge_card_length_chk CHECK (char_length(card) <= 19),
  CONSTRAINT pledge_project_card UNIQUE (project_id, card),
  CONSTRAINT pledge_user_fkey FOREIGN KEY ("user_id") REFERENCES users ("user_id") ON DELETE CASCADE,
  CONSTRAINT pledge_project_fkey FOREIGN KEY ("project_id") REFERENCES projects ("project_id") ON DELETE CASCADE
);
