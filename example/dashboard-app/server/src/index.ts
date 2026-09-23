import express, { Request, Response } from "express";
import cors from "cors";
import { db, initSchema } from "./db";
import {
  InvoiceListItem,
  OverviewResponse,
  PaginatedResponse,
  TimeSeriesPoint,
  GroupBy,
  OrderStatus
} from "./types";

initSchema();

const app = express();
app.use(cors());
app.use(express.json());

const PORT = process.env.PORT ? Number(process.env.PORT) : 4000;

function deriveStatus(quantity: number, customerId: number | null): OrderStatus {
  if (quantity < 0) return "Cancelled";
  if (customerId === null) return "Guest Checkout";
  return "Completed";
}

/**
 * GET /api/invoices
 * Paginated invoice listing with numeric range filters + country + search.
 * Query params:
 *   page (default 1), pageSize (default 10)
 *   minQuantity, maxQuantity, minPrice, maxPrice
 *   country, search (matches Description)
 *   sortBy ('date' | 'quantity' | 'price'), sortDir ('asc' | 'desc')
 */
app.get("/api/invoices", (req: Request, res: Response) => {
  const page = Math.max(1, Number(req.query.page) || 1);
  const pageSize = Math.min(100, Math.max(1, Number(req.query.pageSize) || 10));
  const offset = (page - 1) * pageSize;

  const where: string[] = [];
  const params: Record<string, unknown> = {};

  if (req.query.minQuantity !== undefined) {
    where.push("quantity >= @minQuantity");
    params.minQuantity = Number(req.query.minQuantity);
  }
  if (req.query.maxQuantity !== undefined) {
    where.push("quantity <= @maxQuantity");
    params.maxQuantity = Number(req.query.maxQuantity);
  }
  if (req.query.minPrice !== undefined) {
    where.push("price >= @minPrice");
    params.minPrice = Number(req.query.minPrice);
  }
  if (req.query.maxPrice !== undefined) {
    where.push("price <= @maxPrice");
    params.maxPrice = Number(req.query.maxPrice);
  }
  if (req.query.country) {
    where.push("country = @country");
    params.country = String(req.query.country);
  }
  if (req.query.search) {
    where.push("description LIKE @search");
    params.search = `%${String(req.query.search)}%`;
  }

  const whereClause = where.length ? `WHERE ${where.join(" AND ")}` : "";

  const sortColumn =
    { date: "invoice_date", quantity: "quantity", price: "price" }[
      String(req.query.sortBy)
    ] || "invoice_date";
  const sortDir = String(req.query.sortDir).toUpperCase() === "ASC" ? "ASC" : "DESC";

  const total = (
    db.prepare(`SELECT COUNT(*) AS c FROM invoices ${whereClause}`).get(params) as {
      c: number;
    }
  ).c;

  const rows = db
    .prepare(
      `SELECT invoice AS Invoice, stock_code AS StockCode, description AS Description,
              quantity AS Quantity, invoice_date AS InvoiceDate, price AS Price,
              customer_id AS CustomerID, country AS Country
       FROM invoices
       ${whereClause}
       ORDER BY ${sortColumn} ${sortDir}
       LIMIT @limit OFFSET @offset`
    )
    .all({ ...params, limit: pageSize, offset }) as any[];

  const data: InvoiceListItem[] = rows.map((r) => ({
    ...r,
    LineTotal: Math.round(r.Quantity * r.Price * 100) / 100,
    Status: deriveStatus(r.Quantity, r.CustomerID)
  }));

  const response: PaginatedResponse<InvoiceListItem> = {
    data,
    page,
    pageSize,
    total,
    totalPages: Math.max(1, Math.ceil(total / pageSize))
  };

  res.json(response);
});

/** GET /api/countries — distinct country list, for the filter dropdown */
app.get("/api/countries", (_req: Request, res: Response) => {
  const rows = db
    .prepare("SELECT DISTINCT country FROM invoices ORDER BY country ASC")
    .all() as { country: string }[];
  res.json(rows.map((r) => r.country));
});

/** GET /api/overview — headline stats + status pie + top-country bar data */
app.get("/api/overview", (_req: Request, res: Response) => {
  const rows = db
    .prepare("SELECT quantity, price, customer_id, country FROM invoices")
    .all() as { quantity: number; price: number; customer_id: number | null; country: string }[];

  let totalRevenue = 0;
  let totalUnits = 0;
  const customers = new Set<number>();
  const statusCounts: Record<OrderStatus, number> = {
    Completed: 0,
    Cancelled: 0,
    "Guest Checkout": 0
  };
  const countryRevenue = new Map<string, number>();

  for (const r of rows) {
    const lineTotal = r.quantity * r.price;
    totalRevenue += lineTotal;
    totalUnits += r.quantity;
    if (r.customer_id !== null) customers.add(r.customer_id);

    const status = deriveStatus(r.quantity, r.customer_id);
    statusCounts[status] += 1;

    countryRevenue.set(r.country, (countryRevenue.get(r.country) || 0) + lineTotal);
  }

  const topCountries = Array.from(countryRevenue.entries())
    .map(([country, revenue]) => ({ country, revenue: Math.round(revenue * 100) / 100 }))
    .sort((a, b) => b.revenue - a.revenue)
    .slice(0, 8);

  const response: OverviewResponse = {
    totalRevenue: Math.round(totalRevenue * 100) / 100,
    totalOrders: rows.length,
    totalUnits,
    uniqueCustomers: customers.size,
    statusBreakdown: (Object.keys(statusCounts) as OrderStatus[]).map((status) => ({
      status,
      count: statusCounts[status]
    })),
    topCountries
  };

  res.json(response);
});

/**
 * GET /api/timeseries?groupBy=day|month
 * Revenue + order count bucketed over time.
 */
app.get("/api/timeseries", (req: Request, res: Response) => {
  const groupBy: GroupBy = req.query.groupBy === "day" ? "day" : "month";
  const format = groupBy === "day" ? "%Y-%m-%d" : "%Y-%m";

  const rows = db
    .prepare(
      `SELECT strftime('${format}', invoice_date) AS period,
              SUM(quantity * price) AS revenue,
              COUNT(*) AS orders
       FROM invoices
       GROUP BY period
       ORDER BY period ASC`
    )
    .all() as { period: string; revenue: number; orders: number }[];

  const data: TimeSeriesPoint[] = rows.map((r) => ({
    period: r.period,
    revenue: Math.round(r.revenue * 100) / 100,
    orders: r.orders
  }));

  res.json(data);
});

app.listen(PORT, () => {
  console.log(`API server listening on http://localhost:${PORT}`);
});
