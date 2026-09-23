import { useBridge } from "./BridgeProvider";

export function isQt() {
  return !!window.qt && !!window.bridge;
}

export function qtRequest(path, payload = {}) {

  return new Promise((resolve, reject) => {

    const bridge = window.bridge;
    if (!bridge) {
      return reject(new Error("Qt Bridge is not available"));
    }

    const requestId = crypto.randomUUID();
    const cleanPath = path.replace(/^\/api/, "");

    const payload =
      config.method === "get"
        ? config.params || {}
        : config.data || {};

    // ---------- cleanup ----------
    function cleanup() {
      bridge.progress.disconnect(onProgress);
      bridge.error.disconnect(onError);
      bridge.finished.disconnect(onFinished);
    }

    // ---------- progress ----------
    function onProgress(id, stage) {

      if (id !== requestId) return;

      // handle progress later
    }

    // ---------- error ----------
    function onError(id, payload) {

      if (id !== requestId) return;

      cleanup();
      reject(payload);
    }

    // ---------- success ----------
    function onFinished(id, payload) {
      if (id !== requestId) return;
      cleanup();
      resolve(payload);
    }

    // ---------- subscribe ----------
    bridge.progress.connect(onProgress);
    bridge.error.connect(onError);
    bridge.finished.connect(onFinished);

    // ---------- invoke backend ----------
    bridge.request(
      requestId,
      path,
      JSON.stringify(payload)
    );
  });
}