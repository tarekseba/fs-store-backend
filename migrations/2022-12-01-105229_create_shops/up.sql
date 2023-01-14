create table stores (
  id SERIAL,
  name VARCHAR(256) NOT NULL,
  is_holiday boolean NOT NULL,
  created_at timestamp NOT NULL default now(),
  PRIMARY KEY (id)
);

ALTER TABLE products ADD COLUMN store_id INT;
ALTER TABLE products ADD CONSTRAINT fk_products_stores FOREIGN KEY (store_id) REFERENCES stores (id);
