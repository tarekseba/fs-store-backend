create table worktimes (
  id SERIAL NOT NULL,
  day_id INT NOT NULL,
  store_id INT NOT NULL,
  am_open VARCHAR(5),
  am_close VARCHAR(5),
  pm_open VARCHAR(5),
  pm_close VARCHAR(5),
  CONSTRAINT day_shop UNIQUE (day_id, store_id),
  FOREIGN KEY (store_id) REFERENCES stores(id) ON DELETE CASCADE,
  PRIMARY KEY (id)
)
