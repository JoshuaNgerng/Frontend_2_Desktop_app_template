# Sales Dashboard (React + TypeScript + SQLite)

A small full-stack dashboard: an Express API backed by a SQLite file
(`better-sqlite3`), and a React + TypeScript (Vite) frontend built with
`recharts`.

## Features
- **Paginated invoice listing** — server-side pagination (`page`, `pageSize`), sortable columns (date / quantity / price).
- **Numeric + category filtering** — min/max quantity, min/max price, country dropdown, description search.
- **Time series** — revenue & order count over time, toggle daily/monthly (`recharts` `LineChart`).
- **Overview** — headline stat cards, order-status **pie chart**, and top-countries-by-revenue **bar chart**.

## Project layout
```
dashboard-app/
  data/sample.csv          # the 10-row sample you provided
  server/src/
    db.ts                  # SQLite connection + schema
    seed.ts                # loads data/sample.csv into data/sales.sqlite
    index.ts                # Express API (invoices, countries, overview, timeseries)
    types.ts
  src/
    App.tsx, api.ts, types.ts
    components/
      FilterPanel.tsx, InvoiceTable.tsx, Pagination.tsx
      TimeSeriesChart.tsx, OverviewCharts.tsx
```

## Setup

```bash
npm install

# Seed data/sales.sqlite from data/sample.csv
npm run seed

# Run API (port 4000) + Vite dev server (port 5173) together
npm run dev:all
```

Open http://localhost:5173 — the Vite dev server proxies `/api/*` to the
Express server (see `vite.config.ts`).

## Using your own / full dataset
- Drop your full CSV at `data/sample.csv` (or set `SAMPLE_CSV_PATH`), then
  re-run `npm run seed`.
- Already have a `.sqlite` file? Point `SQLITE_DB_PATH` at it instead of
  seeding — the API reads straight from `data/invoices` table (see
  `server/src/db.ts` for the schema it expects).

## API endpoints
| Endpoint | Description |
|---|---|
| `GET /api/invoices` | Paginated + filtered invoice lines. Query: `page, pageSize, minQuantity, maxQuantity, minPrice, maxPrice, country, search, sortBy, sortDir` |
| `GET /api/countries` | Distinct countries, for the filter dropdown |
| `GET /api/overview` | Total revenue/orders/units/customers, status pie data, top-country bar data |
| `GET /api/timeseries?groupBy=day\|month` | Revenue & order counts bucketed over time |

## Notes on the sample data
The dataset has no explicit "order status" column, so status is derived:
- `Quantity < 0` → **Cancelled**
- `Customer ID` missing → **Guest Checkout**
- otherwise → **Completed**

Adjust `deriveStatus()` in `server/src/index.ts` if your real data has an
actual status/refund flag.

## Production build
```bash
npm run build          # frontend -> dist/
npm run server:build   # backend  -> dist-server/
```
Serve `dist/` as static files behind the compiled `dist-server/index.js`,
or put both behind a reverse proxy.
