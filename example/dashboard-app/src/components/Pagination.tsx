import React from "react";

interface Props {
  page: number;
  totalPages: number;
  total: number;
  onPageChange: (page: number) => void;
}

const Pagination: React.FC<Props> = ({ page, totalPages, total, onPageChange }) => {
  const pages: number[] = [];
  const windowSize = 2;
  for (
    let p = Math.max(1, page - windowSize);
    p <= Math.min(totalPages, page + windowSize);
    p++
  ) {
    pages.push(p);
  }

  return (
    <div className="pagination">
      <button
        className="page-btn"
        disabled={page <= 1}
        onClick={() => onPageChange(page - 1)}
      >
        Prev
      </button>

      {pages[0] > 1 && <span>…</span>}
      {pages.map((p) => (
        <button
          key={p}
          className={`page-btn ${p === page ? "active" : ""}`}
          onClick={() => onPageChange(p)}
        >
          {p}
        </button>
      ))}
      {pages[pages.length - 1] < totalPages && <span>…</span>}

      <button
        className="page-btn"
        disabled={page >= totalPages}
        onClick={() => onPageChange(page + 1)}
      >
        Next
      </button>

      <span style={{ marginLeft: "auto", color: "var(--text-dim)" }}>
        {total} total rows · page {page} of {totalPages}
      </span>
    </div>
  );
};

export default Pagination;
