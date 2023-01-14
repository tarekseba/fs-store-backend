CREATE TABLE products (
  id SERIAL,
  name VARCHAR(256) NOT NULL,
  price NUMERIC(9, 2) NOT NULL,
  description TEXT,
  created_at timestamp NOT NULL default now(),
  PRIMARY KEY (id)
)
