import React, { createContext, useContext, useEffect, useState } from "react";

const BridgeContext = createContext(null);

export function BridgeProvider({ children }) {
  const [bridge, setBridge] = useState(null);
  const [ready, setReady] = useState(false);
  const [error, setError] = useState(null);

  console.log('test start bridge provider');
  useEffect(() => {
    let cancelled = false;

    function initBridge() {
      try {
        if (!window.qt || !window.qt.webChannelTransport) {
          console.warn("Qt transport not ready yet");
          return false;
        }

        if (!window.QWebChannel) {
          console.error("QWebChannel script not loaded");
          return false;
        }

        new window.QWebChannel(window.qt.webChannelTransport, (channel) => {
          if (cancelled) return;

          const bridgeObj = channel.objects.bridge;

          if (!bridgeObj) {
            setError("Bridge object not found (check Qt side name)");
            return;
          }

          window.bridge = bridgeObj; // optional global access
          setBridge(bridgeObj);
          setReady(true);

          console.log("Qt bridge connected");
        });

        return true;
      } catch (e) {
        setError(e.message);
        return false;
      }
    }

    // Retry loop (important in Qt apps)
    const interval = setInterval(() => {
      const success = initBridge();
      if (success) clearInterval(interval);
    }, 200);

    // cleanup
    return () => {
      cancelled = true;
      clearInterval(interval);
    };
  }, []);

  return (
    <BridgeContext.Provider value={{ bridge, ready, error }}>
      {children}
    </BridgeContext.Provider>
  );
}

export function useBridge() {
  return useContext(BridgeContext);
}