import React, { useEffect, useState } from "react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer
} from "recharts";
import { fetchTimeSeries } from "../api";
import { GroupBy, TimeSeriesPoint } from "../types";

const TimeSeriesChart: React.FC = () => {
  const [groupBy, setGroupBy] = useState<GroupBy>("month");
  const [data, setData] = useState<TimeSeriesPoint[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    fetchTimeSeries(groupBy)
      .then((res) => {
        if (!cancelled) setData(res);
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => {
      cancelled = true;
    };
  }, [groupBy]);

  return (
    <div className="panel">
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
        <h2>Revenue &amp; Orders Over Time</h2>
        <div className="toggle-group">
          <button
            className={`btn ${groupBy === "day" ? "" : "secondary"}`}
            onClick={() => setGroupBy("day")}
          >
            Daily
          </button>
          <button
            className={`btn ${groupBy === "month" ? "" : "secondary"}`}
            onClick={() => setGroupBy("month")}
          >
            Monthly
          </button>
        </div>
      </div>

      {loading ? (
        <div className="empty-state">Loading time series…</div>
      ) : data.length === 0 ? (
        <div className="empty-state">No data for this range</div>
      ) : (
        <ResponsiveContainer width="100%" height={280}>
          <LineChart data={data} margin={{ top: 10, right: 20, left: 0, bottom: 0 }}>
            <CartesianGrid strokeDasharray="3 3" stroke="#334155" />
            <XAxis dataKey="period" stroke="#94a3b8" fontSize={12} />
            <YAxis yAxisId="left" stroke="#38bdf8" fontSize={12} />
            <YAxis yAxisId="right" orientation="right" stroke="#a78bfa" fontSize={12} />
            <Tooltip contentStyle={{ background: "#1e293b", border: "1px solid #334155" }} />
            <Legend />
            <Line
              yAxisId="left"
              type="monotone"
              dataKey="revenue"
              stroke="#38bdf8"
              strokeWidth={2}
              dot={false}
              name="Revenue"
            />
            <Line
              yAxisId="right"
              type="monotone"
              dataKey="orders"
              stroke="#a78bfa"
              strokeWidth={2}
              dot={false}
              name="Orders"
            />
          </LineChart>
        </ResponsiveContainer>
      )}
    </div>
  );
};

export default TimeSeriesChart;
