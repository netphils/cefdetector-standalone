import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import background from "./assets/background.png";
import "./ResultsPage.css";

type AppEntry = {
  icon: string;
  name: string;
  appType: string;
  size: string;
};

export default function ResultsPage() {
  const [entries, setEntries] = useState<AppEntry[]>([]);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    let cancelled = false;

    const start = async () => {
      unlisten = await listen<AppEntry>("app-entry", (event) => {
        setEntries((prev) => [...prev, event.payload]);
      });
      if (cancelled) {
        unlisten();
        return;
      }
      await invoke("scan_apps", { params: {} });
    };
    start();

    return () => {
      cancelled = true;
      unlisten?.();
    };
  }, []);

  return (
    <div
      className="results"
      style={{ backgroundImage: `url(${background})` }}
    >
      <div className="results-top">
        <h2 className="results-header">
          恭喜你，已找到 {entries.length} 个浏览器应用
        </h2>
      </div>

      <div className="results-grid">
        {entries.map((entry, i) => (
          <div className="result-card" key={i}>
            <img
              className="result-icon"
              src={`data:image/png;base64,${entry.icon}`}
              alt=""
            />
            <div className="result-name">{entry.name}</div>
            <div className="result-type">{entry.appType}</div>
            <div className="result-size">{entry.size}</div>
          </div>
        ))}
      </div>
    </div>
  );
}
