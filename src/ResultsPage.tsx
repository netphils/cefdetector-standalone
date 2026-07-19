import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import background from "./assets/background.png";
import noImage from "./assets/no-image-svgrepo-com.svg";
import "./ResultsPage.css";

type AppEntry = {
  icon: string;
  name: string;
  appType: string;
  size: string;
};

export default function ResultsPage() {
  const [entries, setEntries] = useState<AppEntry[]>([]);
  const [totalSize, setTotalSize] = useState<string>("");

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
      const result = (await invoke("scan_apps", {
        params: {},
      })) as Record<string, string>;
      if (!cancelled) {
        setTotalSize(result.size ?? "");
      }
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
          恭喜你，这台电脑上总共有 {entries.length} 个 Chromium 内核的应用
          {totalSize && `（共 ${totalSize}）`}
        </h2>
      </div>

      <div className="results-grid">
        {entries.map((entry, i) => (
          <div className="result-card" key={i}>
            <img
              className="result-icon"
              src={entry.icon ? `data:image/png;base64,${entry.icon}` : noImage}
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
