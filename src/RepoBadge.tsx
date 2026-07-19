import { useEffect, useState } from "react";
import { getVersion, getTauriVersion } from "@tauri-apps/api/app";
import githubIcon from "./assets/github-142-svgrepo-com.svg";
import "./RepoBadge.css";

const REPO_URL = "https://github.com/netphils/cefdetector-standalone";

export default function RepoBadge() {
  const [open, setOpen] = useState(false);
  const [appVersion, setAppVersion] = useState("");
  const [tauriVersion, setTauriVersion] = useState("");

  useEffect(() => {
    if (!open) return;
    getVersion().then(setAppVersion).catch(() => setAppVersion("未知"));
    getTauriVersion().then(setTauriVersion).catch(() => setTauriVersion("未知"));
  }, [open]);

  return (
    <>
      <button
        className="repo-badge"
        onClick={() => setOpen(true)}
        aria-label="仓库地址"
      >
        <img src={githubIcon} alt="" />
      </button>

      {open && (
        <div className="repo-modal-overlay" onClick={() => setOpen(false)}>
          <div className="repo-modal" onClick={(e) => e.stopPropagation()}>
            <h3 className="repo-modal-title">仓库地址</h3>
            <a className="repo-modal-link" href={REPO_URL} target="_blank" rel="noreferrer">
              {REPO_URL}
            </a>
            <div className="repo-modal-versions">
              <span>应用版本：{appVersion || "加载中…"}</span>
              <span>Tauri 版本：{tauriVersion || "加载中…"}</span>
            </div>
            <button className="repo-modal-close" onClick={() => setOpen(false)}>
              关闭
            </button>
          </div>
        </div>
      )}
    </>
  );
}
