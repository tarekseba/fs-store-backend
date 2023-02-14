create table stores (
  id SERIAL,
  name VARCHAR(256) NOT NULL,
  is_holiday boolean NOT NULL,
  created_at timestamp NOT NULL default now(),
  prod_count INT NOT NULL default 0,
  PRIMARY KEY (id)
);

ALTER TABLE products ADD COLUMN store_id INT;
ALTER TABLE products ADD CONSTRAINT fk_products_stores FOREIGN KEY (store_id) REFERENCES stores (id);

GRANT USAGE ON LANGUAGE plpgsql TO myusername;

CREATE OR REPLACE FUNCTION update_store_product_count()
RETURNS TRIGGER
AS $$
BEGIN
  IF OLD.store_id IS NULL AND NEW.store_id IS NOT NULL THEN
    UPDATE stores SET prod_count = prod_count + 1 WHERE id = NEW.store_id;
  ELSEIF OLD.store_id IS NOT NULL AND NEW.store_id IS NOT NULL THEN
    UPDATE stores SET prod_count = prod_count + 1 WHERE id = NEW.store_id;
    UPDATE stores SET prod_count = prod_count - 1 WHERE id = OLD.store_id AND prod_count > 0;
  ELSEIF OLD.store_id IS NOT NULL AND NEW.store_id IS NULL THEN
    UPDATE stores SET prod_count = prod_count - 1 WHERE id = OLD.store_id AND prod_count > 0;
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE PLPGSQL;

CREATE OR REPLACE TRIGGER increment_prod_count_trigger AFTER UPDATE ON products FOR ROW EXECUTE PROCEDURE update_store_product_count();
