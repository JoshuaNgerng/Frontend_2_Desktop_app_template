import React from 'react';
import ReactDOM from 'react-dom/client';

import App from './App';
import { BridgeProvider } from './api/BridgeProvider';

const root = ReactDOM.createRoot(
  document.getElementById('root')
);

root.render(
  <React.StrictMode>
    <BridgeProvider>
      <App />
    </BridgeProvider>
  </React.StrictMode>
);