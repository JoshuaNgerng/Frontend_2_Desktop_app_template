import {
  GroupBy,
  InvoiceFilters,
  InvoiceListItem,
  OverviewResponse,
  PaginatedResponse,
  TimeSeriesPoint
} from "./types";

const BASE = "/api";

function buildQuery(params: Record<string, unknown>): string {
  const search = new URLSearchParams();
  Object.entries(params).forEach(([key, value]) => {
    if (value !== undefined && value !== null && value !== "") {
      search.set(key, String(value));
    }
  });
  return search.toString();
}

async function getJson<T>(url: string): Promise<T> {
  const res = await fetch(url);
  if (!res.ok) throw new Error(`Request failed: ${res.status} ${res.statusText}`);
  return res.json() as Promise<T>;
}

export function fetchInvoices(
  filters: InvoiceFilters
): Promise<PaginatedResponse<InvoiceListItem>> {
  return getJson(`${BASE}/invoices?${buildQuery(filters)}`);
}

export function fetchCountries(): Promise<string[]> {
  return getJson(`${BASE}/countries`);
}

export function fetchOverview(): Promise<OverviewResponse> {
  return getJson(`${BASE}/overview`);
}

export function fetchTimeSeries(groupBy: GroupBy): Promise<TimeSeriesPoint[]> {
  return getJson(`${BASE}/timeseries?${buildQuery({ groupBy })}`);
}
