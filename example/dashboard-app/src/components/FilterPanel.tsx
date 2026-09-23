import React from "react";
import { InvoiceFilters } from "../types";

interface Props {
  filters: InvoiceFilters;
  countries: string[];
  onChange: (patch: Partial<InvoiceFilters>) => void;
  onReset: () => void;
}

const FilterPanel: React.FC<Props> = ({ filters, countries, onChange, onReset }) => {
  const num = (v: string): number | undefined => (v === "" ? undefined : Number(v));

  return (
    <div className="panel">
      <h2>Filters</h2>
      <div className="filter-row">
        <div className="field">
          <label>Description search</label>
          <input
            type="text"
            placeholder="e.g. mug"
            value={filters.search ?? ""}
            onChange={(e) => onChange({ search: e.target.value, page: 1 })}
          />
        </div>

        <div className="field">
          <label>Min quantity</label>
          <input
            type="number"
            value={filters.minQuantity ?? ""}
            onChange={(e) => onChange({ minQuantity: num(e.target.value), page: 1 })}
          />
        </div>
        <div className="field">
          <label>Max quantity</label>
          <input
            type="number"
            value={filters.maxQuantity ?? ""}
            onChange={(e) => onChange({ maxQuantity: num(e.target.value), page: 1 })}
          />
        </div>

        <div className="field">
          <label>Min price</label>
          <input
            type="number"
            step="0.01"
            value={filters.minPrice ?? ""}
            onChange={(e) => onChange({ minPrice: num(e.target.value), page: 1 })}
          />
        </div>
        <div className="field">
          <label>Max price</label>
          <input
            type="number"
            step="0.01"
            value={filters.maxPrice ?? ""}
            onChange={(e) => onChange({ maxPrice: num(e.target.value), page: 1 })}
          />
        </div>

        <div className="field">
          <label>Country</label>
          <select
            value={filters.country ?? ""}
            onChange={(e) => onChange({ country: e.target.value || undefined, page: 1 })}
          >
            <option value="">All</option>
            {countries.map((c) => (
              <option key={c} value={c}>
                {c}
              </option>
            ))}
          </select>
        </div>

        <div className="field">
          <label>Page size</label>
          <select
            value={filters.pageSize}
            onChange={(e) => onChange({ pageSize: Number(e.target.value), page: 1 })}
          >
            {[10, 25, 50, 100].map((n) => (
              <option key={n} value={n}>
                {n}
              </option>
            ))}
          </select>
        </div>

        <button className="btn secondary" onClick={onReset}>
          Reset
        </button>
      </div>
    </div>
  );
};

export default FilterPanel;
