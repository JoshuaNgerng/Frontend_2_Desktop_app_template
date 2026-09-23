import Database from "better-sqlite3";
import path from "path";
import fs from "fs";

// The SQLite file that stores all invoice line data.
// Point SQLITE_DB_PATH at an existing .sqlite file to reuse one you already have.
const DB_PATH =
  process.env.SQLITE_DB_PATH || path.join(process.cwd(), "data", "sales.sqlite");

fs.mkdirSync(path.dirname(DB_PATH), { recursive: true });

export const db = new Database(DB_PATH);
db.pragma("journal_mode = WAL");

export function initSchema(): void {
  db.exec(`
    CREATE TABLE IF NOT EXISTS invoices (
      id            INTEGER PRIMARY KEY AUTOINCREMENT,
      invoice       TEXT NOT NULL,
      stock_code    TEXT NOT NULL,
      description   TEXT NOT NULL,
      quantity      INTEGER NOT NULL,
      invoice_date  TEXT NOT NULL,     -- ISO 8601 timestamp
      price         REAL NOT NULL,
      customer_id   INTEGER,           -- nullable, matches source data
      country       TEXT NOT NULL
    );

    CREATE INDEX IF NOT EXISTS idx_invoices_date     ON invoices (invoice_date);
    CREATE INDEX IF NOT EXISTS idx_invoices_country   ON invoices (country);
    CREATE INDEX IF NOT EXISTS idx_invoices_quantity  ON invoices (quantity);
    CREATE INDEX IF NOT EXISTS idx_invoices_price     ON invoices (price);
  `);
}
