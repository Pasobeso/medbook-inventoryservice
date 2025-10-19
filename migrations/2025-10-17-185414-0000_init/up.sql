-- Your SQL goes here

CREATE TABLE "inventory" (
  "product_id" integer PRIMARY KEY,
  "total_quantity" integer NOT NULL DEFAULT 0,
  "reserved_quantity" integer NOT NULL DEFAULT 0,
  "sold_quantity" integer NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_inventory_timestamp
BEFORE UPDATE ON inventory
FOR EACH ROW
EXECUTE FUNCTION diesel_set_updated_at();

CREATE TABLE "products" (
  "id" serial PRIMARY KEY,
  "th_name" text NOT NULL DEFAULT 'ชื่อผลิตภัณฑ์',
  "en_name" text NOT NULL DEFAULT 'Product Name',
  "unit_price" real NOT NULL DEFAULT 0.00,
  "image_path" varchar(255),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_product_timestamp
BEFORE UPDATE ON products
FOR EACH ROW
EXECUTE FUNCTION diesel_set_updated_at();

ALTER TABLE "inventory" ADD FOREIGN KEY ("product_id") REFERENCES "products" ("id") ON DELETE CASCADE;

-- Seed Products with Mock Prices
INSERT INTO products (id, th_name, en_name, unit_price) VALUES
    (1,  'พาราเซตามอล', 'Paracetamol', 25.00),
    (2,  'ไอบูโพรเฟน', 'Ibuprofen', 45.00),
    (3,  'แอสไพริน', 'Aspirin',  35.00),
    (4,  'คลอร์เฟนิรามีน', 'Chlorpheniramine', 15.00),
    (5,  'ลอราทาดีน', 'Loratadine', 30.00),
    (6,  'ซิโปรฟลอกซาซิน', 'Ciprofloxacin', 90.00),
    (7,  'อะม็อกซิซิลลิน', 'Amoxicillin', 60.00),
    (8,  'โซเดียมไบคาร์บอเนต', 'Sodium Bicarbonate', 20.00),
    (9,  'โอเมพราโซล', 'Omeprazole', 55.00),
    (10, 'เกลือแร่', 'Oral Rehydration Salts', 12.00);

-- Seed Inventory
INSERT INTO inventory (product_id, total_quantity) VALUES
    (1, 10);
