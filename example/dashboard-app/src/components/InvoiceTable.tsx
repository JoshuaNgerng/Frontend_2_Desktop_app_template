import React from "react";
import { InvoiceFilters, InvoiceListItem } from "../types";

interface Props {
  rows: InvoiceListItem[];
  loading: boolean;
  filters: InvoiceFilters;
  onSort: (sortBy: InvoiceFilters["sortBy"]) => void;
}

const InvoiceTable: React.FC<Props> = ({ rows, loading, filters, onSort }) => {
  const arrow = (col: InvoiceFilters["sortBy"]) =>
    filters.sortBy === col ? (filters.sortDir === "asc" ? " ▲" : " ▼") : "";

  return (
    <div className="panel">
      <h2>Invoice Line Items</h2>
      {loading ? (
        <div className="empty-state">Loading…</div>
      ) : rows.length === 0 ? (
        <div className="empty-state">No rows match the current filters.</div>
      ) : (
        <table>
          <thead>
            <tr>
              <th>Invoice</th>
              <th>Stock Code</th>
              <th>Description</th>
              <th onClick={() => onSort("quantity")}>Qty{arrow("quantity")}</th>
              <th onClick={() => onSort("date")}>Date{arrow("date")}</th>
              <th onClick={() => onSort("price")}>Price{arrow("price")}</th>
              <th>Line Total</th>
              <th>Customer ID</th>
              <th>Country</th>
              <th>Status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((r, i) => (
              <tr key={`${r.Invoice}-${r.StockCode}-${i}`}>
                <td>{r.Invoice}</td>
                <td>{r.StockCode}</td>
                <td>{r.Description}</td>
                <td>{r.Quantity}</td>
                <td>{new Date(r.InvoiceDate).toLocaleString()}</td>
                <td>£{r.Price.toFixed(2)}</td>
                <td>£{r.LineTotal.toFixed(2)}</td>
                <td>{r.CustomerID ?? "—"}</td>
                <td>{r.Country}</td>
                <td>
                  <span className={`badge ${r.Status.replace(" ", "")}`}>{r.Status}</span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
};

export default InvoiceTable;
