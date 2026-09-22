import { useState } from "react";
import './App.css';
import Dashboard from './pages/Dashboard';
import Records   from './pages/Records';
import Upload    from './pages/Upload';
import { ToastContainer } from './components/Toast';
import { useStats } from './hooks/useStats.js';

const NAV = [
  { id: 'dashboard', label: 'Dashboard',   icon: '◈' },
  { id: 'records',   label: 'Records',     icon: '≡' },
  { id: 'upload',    label: 'Upload Data', icon: '↑' },
];

export default function App() {
  const [page, setPage] = useState('dashboard');
  const { refresh }     = useStats();

  const renderPage = () => {
    if (page === 'dashboard') return <Dashboard />;
    if (page === 'records')   return <Records />;
    if (page === 'upload')    return <Upload onUploadSuccess={() => { refresh(); }} />;
    return null;
  };

  return (
    <div className="app-shell">
      {/* Sidebar */}
      <aside className="sidebar">
        <div className="sidebar-logo">
          <div className="brand">Am<em>Bank</em></div>
          <div className="sub">HP Portfolio · AI CoE</div>
        </div>
        <nav className="sidebar-nav">
          {NAV.map(n => (
            <div
              key={n.id}
              className={`nav-item ${page === n.id ? 'active' : ''}`}
              onClick={() => setPage(n.id)}
            >
              <span className="nav-icon">{n.icon}</span>
              {n.label}
            </div>
          ))}
        </nav>
        <div className="sidebar-footer">
          RCR HP Portfolio<br />
          Analytics v1.0<br />
          AmBank Group © 2025
        </div>
      </aside>

      {/* Main */}
      <main className="main">
        {renderPage()}
      </main>

      <ToastContainer />
    </div>
  );
}
