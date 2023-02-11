CREATE TABLE products (
  id SERIAL,
  name VARCHAR(256) NOT NULL,
  i18n_name VARCHAR(256),
  price NUMERIC(9, 2) NOT NULL,
  description TEXT,
  i18n_description TEXT,
  created_at timestamp NOT NULL default now(),
  PRIMARY KEY (id)
)
