CREATE TABLE categories (
  id SERIAL,
  name VARCHAR(256) UNIQUE NOT NULL,
  created_at timestamp NOT NULL default now(),
  PRIMARY KEY (id)
);
INSERT INTO categories (name) values ('Tarek the best ?');
