import React from "react";
import {
  PieChart,
  Pie,
  Cell,
  Tooltip,
  Legend,
  ResponsiveContainer,
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid
} from "recharts";
import { OverviewResponse } from "../types";

interface Props {
  overview: OverviewResponse | null;
  loading: boolean;
}

const STATUS_COLORS: Record<string, string> = {
  Completed: "#4ade80",
  Cancelled: "#f87171",
  "Guest Checkout": "#a78bfa"
};

const BAR_COLOR = "#38bdf8";

const OverviewCharts: React.FC<Props> = ({ overview, loading }) => {
  if (loading || !overview) {
    return <div className="panel empty-state">Loading overview…</div>;
  }

  const pieData = overview.statusBreakdown.filter((s) => s.count > 0);

  return (
    <div className="charts-grid">
      <div className="panel">
        <h2>Order Status Breakdown</h2>
        {pieData.length === 0 ? (
          <div className="empty-state">No data</div>
        ) : (
          <ResponsiveContainer width="100%" height={260}>
            <PieChart>
              <Pie
                data={pieData}
                dataKey="count"
                nameKey="status"
                cx="50%"
                cy="50%"
                outerRadius={90}
                label={(entry) => `${entry.status} (${entry.count})`}
              >
                {pieData.map((entry) => (
                  <Cell key={entry.status} fill={STATUS_COLORS[entry.status] || "#38bdf8"} />
                ))}
              </Pie>
              <Tooltip
                contentStyle={{ background: "#1e293b", border: "1px solid #334155" }}
              />
              <Legend />
            </PieChart>
          </ResponsiveContainer>
        )}
      </div>

      <div className="panel">
        <h2>Revenue by Country (Top 8)</h2>
        {overview.topCountries.length === 0 ? (
          <div className="empty-state">No data</div>
        ) : (
          <ResponsiveContainer width="100%" height={260}>
            <BarChart data={overview.topCountries} layout="vertical" margin={{ left: 20 }}>
              <CartesianGrid strokeDasharray="3 3" stroke="#334155" />
              <XAxis type="number" stroke="#94a3b8" fontSize={12} />
              <YAxis
                type="category"
                dataKey="country"
                stroke="#94a3b8"
                fontSize={12}
                width={90}
              />
              <Tooltip contentStyle={{ background: "#1e293b", border: "1px solid #334155" }} />
              <Bar dataKey="revenue" fill={BAR_COLOR} radius={[0, 4, 4, 0]} />
            </BarChart>
          </ResponsiveContainer>
        )}
      </div>
    </div>
  );
};

export default OverviewCharts;
