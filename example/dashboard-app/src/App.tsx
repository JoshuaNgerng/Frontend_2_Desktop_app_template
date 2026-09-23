import React, { useEffect, useState, useCallback } from "react";
import { fetchCountries, fetchInvoices, fetchOverview } from "./api";
import { InvoiceFilters, InvoiceListItem, OverviewResponse } from "./types";
import FilterPanel from "./components/FilterPanel";
import InvoiceTable from "./components/InvoiceTable";
import Pagination from "./components/Pagination";
import TimeSeriesChart from "./components/TimeSeriesChart";
import OverviewCharts from "./components/OverviewCharts";

const DEFAULT_FILTERS: InvoiceFilters = {
  page: 1,
  pageSize: 10,
  sortBy: "date",
  sortDir: "desc"
};

const StatCard: React.FC<{ label: string; value: string }> = ({ label, value }) => (
  <div className="stat-card">
    <div className="label">{label}</div>
    <div className="value">{value}</div>
  </div>
);

const App: React.FC = () => {
  const [filters, setFilters] = useState<InvoiceFilters>(DEFAULT_FILTERS);
  const [rows, setRows] = useState<InvoiceListItem[]>([]);
  const [total, setTotal] = useState(0);
  const [totalPages, setTotalPages] = useState(1);
  const [rowsLoading, setRowsLoading] = useState(true);

  const [countries, setCountries] = useState<string[]>([]);
  const [overview, setOverview] = useState<OverviewResponse | null>(null);
  const [overviewLoading, setOverviewLoading] = useState(true);

  const loadInvoices = useCallback((f: InvoiceFilters) => {
    setRowsLoading(true);
    fetchInvoices(f)
      .then((res) => {
        setRows(res.data);
        setTotal(res.total);
        setTotalPages(res.totalPages);
      })
      .finally(() => setRowsLoading(false));
  }, []);

  useEffect(() => {
    loadInvoices(filters);
  }, [filters, loadInvoices]);

  useEffect(() => {
    fetchCountries().then(setCountries);
    setOverviewLoading(true);
    fetchOverview()
      .then(setOverview)
      .finally(() => setOverviewLoading(false));
  }, []);

  const handleFilterChange = (patch: Partial<InvoiceFilters>) => {
    setFilters((prev) => ({ ...prev, ...patch }));
  };

  const handleReset = () => setFilters(DEFAULT_FILTERS);

  const handleSort = (sortBy: InvoiceFilters["sortBy"]) => {
    setFilters((prev) => ({
      ...prev,
      sortBy,
      sortDir: prev.sortBy === sortBy && prev.sortDir === "desc" ? "asc" : "desc"
    }));
  };

  return (
    <div className="dashboard">
      <h1>Sales Dashboard</h1>
      <p className="subtitle">Backed by a SQLite file, served through a small Express API.</p>

      <div className="stat-grid">
        <StatCard
          label="Total Revenue"
          value={overviewLoading || !overview ? "…" : `£${overview.totalRevenue.toFixed(2)}`}
        />
        <StatCard
          label="Total Orders"
          value={overviewLoading || !overview ? "…" : String(overview.totalOrders)}
        />
        <StatCard
          label="Units Sold"
          value={overviewLoading || !overview ? "…" : String(overview.totalUnits)}
        />
        <StatCard
          label="Unique Customers"
          value={overviewLoading || !overview ? "…" : String(overview.uniqueCustomers)}
        />
      </div>

      <OverviewCharts overview={overview} loading={overviewLoading} />

      <TimeSeriesChart />

      <FilterPanel
        filters={filters}
        countries={countries}
        onChange={handleFilterChange}
        onReset={handleReset}
      />

      <InvoiceTable rows={rows} loading={rowsLoading} filters={filters} onSort={handleSort} />

      <Pagination
        page={filters.page}
        totalPages={totalPages}
        total={total}
        onPageChange={(page) => setFilters((prev) => ({ ...prev, page }))}
      />
    </div>
  );
};

export default App;
