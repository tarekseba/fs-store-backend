CREATE TABLE products_categories (
  id SERIAL,
  product_id INT NOT NULL,
  category_id INT NOT NULL,
  FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
  FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE,
  CONSTRAINT prod_cat UNIQUE (product_id, category_id),
  PRIMARY KEY (id)
)
