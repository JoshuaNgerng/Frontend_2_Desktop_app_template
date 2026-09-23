export interface InvoiceRow {
  Invoice: string;
  StockCode: string;
  Description: string;
  Quantity: number;
  InvoiceDate: string; // ISO string
  Price: number;
  CustomerID: number | null;
  Country: string;
}

export type OrderStatus = "Completed" | "Cancelled" | "Guest Checkout";

export interface InvoiceListItem extends InvoiceRow {
  LineTotal: number;
  Status: OrderStatus;
}

export interface PaginatedResponse<T> {
  data: T[];
  page: number;
  pageSize: number;
  total: number;
  totalPages: number;
}

export interface OverviewResponse {
  totalRevenue: number;
  totalOrders: number;
  totalUnits: number;
  uniqueCustomers: number;
  statusBreakdown: { status: OrderStatus; count: number }[];
  topCountries: { country: string; revenue: number }[];
}

export interface TimeSeriesPoint {
  period: string; // e.g. "2010-11" or "2010-11-02"
  revenue: number;
  orders: number;
}

export type GroupBy = "day" | "month";
