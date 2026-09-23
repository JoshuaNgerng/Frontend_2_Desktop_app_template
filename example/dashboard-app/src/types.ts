export type OrderStatus = "Completed" | "Cancelled" | "Guest Checkout";

export interface InvoiceListItem {
  Invoice: string;
  StockCode: string;
  Description: string;
  Quantity: number;
  InvoiceDate: string;
  Price: number;
  CustomerID: number | null;
  Country: string;
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
  period: string;
  revenue: number;
  orders: number;
}

export type GroupBy = "day" | "month";

export interface InvoiceFilters {
  page: number;
  pageSize: number;
  minQuantity?: number;
  maxQuantity?: number;
  minPrice?: number;
  maxPrice?: number;
  country?: string;
  search?: string;
  sortBy?: "date" | "quantity" | "price";
  sortDir?: "asc" | "desc";
}
