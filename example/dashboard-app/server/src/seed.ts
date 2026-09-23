/**
 * Loads data/sample.csv into the SQLite database.
 * Run with: npm run seed
 *
 * Swap SAMPLE_CSV_PATH / SQLITE_DB_PATH env vars to point this at your full
 * dataset instead of the 10-row sample.
 */
import fs from "fs";
import path from "path";
import { db, initSchema } from "./db";

const CSV_PATH =
  process.env.SAMPLE_CSV_PATH || path.join(process.cwd(), "data", "sample.csv");

// Minimal CSV parser that handles quoted fields containing commas
// (good enough for this dataset; swap for `csv-parse` for very large/odd files).
function parseCsv(text: string): string[][] {
  const rows: string[][] = [];
  let row: string[] = [];
  let field = "";
  let inQuotes = false;

  for (let i = 0; i < text.length; i++) {
    const char = text[i];
    const next = text[i + 1];

    if (inQuotes) {
      if (char === '"' && next === '"') {
        field += '"';
        i++;
      } else if (char === '"') {
        inQuotes = false;
      } else {
        field += char;
      }
    } else {
      if (char === '"') {
        inQuotes = true;
      } else if (char === ",") {
        row.push(field);
        field = "";
      } else if (char === "\n") {
        if (field.length > 0 || row.length > 0) {
          row.push(field.replace(/\r$/, ""));
          rows.push(row);
        }
        row = [];
        field = "";
      } else {
        field += char;
      }
    }
  }
  if (field.length > 0 || row.length > 0) {
    row.push(field);
    rows.push(row);
  }
  return rows;
}

function seed(): void {
  initSchema();

  const raw = fs.readFileSync(CSV_PATH, "utf-8");
  const rows = parseCsv(raw);
  const [header, ...records] = rows;

  const col = (name: string) => header.indexOf(name);
  const idx = {
    Invoice: col("Invoice"),
    StockCode: col("StockCode"),
    Description: col("Description"),
    Quantity: col("Quantity"),
    InvoiceDate: col("InvoiceDate"),
    Price: col("Price"),
    CustomerID: col("Customer ID"),
    Country: col("Country")
  };

  db.exec("DELETE FROM invoices"); // idempotent re-seed

  const insert = db.prepare(`
    INSERT INTO invoices (invoice, stock_code, description, quantity, invoice_date, price, customer_id, country)
    VALUES (@invoice, @stock_code, @description, @quantity, @invoice_date, @price, @customer_id, @country)
  `);

  const insertMany = db.transaction((items: any[]) => {
    for (const item of items) insert.run(item);
  });

  const values = records
    .filter((r) => r.length >= header.length && r[idx.Invoice])
    .map((r) => ({
      invoice: r[idx.Invoice],
      stock_code: r[idx.StockCode],
      description: r[idx.Description].trim(),
      quantity: Number(r[idx.Quantity]),
      invoice_date: r[idx.InvoiceDate],
      price: Number(r[idx.Price]),
      customer_id: r[idx.CustomerID] ? Number(r[idx.CustomerID]) : null,
      country: r[idx.Country]
    }));

  insertMany(values);

  console.log(`Seeded ${values.length} rows into ${process.env.SQLITE_DB_PATH || "data/sales.sqlite"}`);
}

seed();
