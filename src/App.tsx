import { useEffect, useRef, useState } from "react";
import browserIcon from "./assets/browser-chrome-google-svgrepo-com.svg";
import RepoBadge from "./RepoBadge";
import "./App.css";

const ICON_COUNT = 10;
const ICON_SIZE = 56;

type Floater = {
  id: number;
  x: number;
  y: number;
  vx: number;
  vy: number;
};

const rand = (min: number, max: number) => min + Math.random() * (max - min);

function App() {
  const [floaters, setFloaters] = useState<Floater[]>([]);
  const areaRef = useRef<HTMLDivElement>(null);
  const floatersRef = useRef<Floater[]>([]);
  const [started, setStarted] = useState(false);

  useEffect(() => {
    const area = areaRef.current;
    if (!area) return;

    const { width, height } = area.getBoundingClientRect();
    const initial: Floater[] = Array.from({ length: ICON_COUNT }, (_, id) => {
      const angle = rand(0, Math.PI * 2);
      const speed = rand(0.4, 1.1);
      return {
        id,
        x: rand(0, Math.max(0, width - ICON_SIZE)),
        y: rand(0, Math.max(0, height - ICON_SIZE)),
        vx: Math.cos(angle) * speed,
        vy: Math.sin(angle) * speed,
      };
    });

    floatersRef.current = initial;
    setFloaters(initial);

    let raf = 0;
    const tick = () => {
      const rect = area.getBoundingClientRect();
      const w = rect.width;
      const h = rect.height;
      floatersRef.current = floatersRef.current.map((f) => {
        let { x, y, vx, vy } = f;
        x += vx;
        y += vy;
        if (x <= 0) {
          x = 0;
          vx = -vx;
        } else if (x >= w - ICON_SIZE) {
          x = w - ICON_SIZE;
          vx = -vx;
        }
        if (y <= 0) {
          y = 0;
          vy = -vy;
        } else if (y >= h - ICON_SIZE) {
          y = h - ICON_SIZE;
          vy = -vy;
        }
        return { ...f, x, y, vx, vy };
      });
      setFloaters(floatersRef.current.map((f) => ({ ...f })));
      raf = requestAnimationFrame(tick);
    };
    raf = requestAnimationFrame(tick);

    return () => cancelAnimationFrame(raf);
  }, []);

  return (
    <div className="home" ref={areaRef} data-started={started}>
      <h1 className="home-title">CEFDetector-Standalone</h1>
      {floaters.map((f) => (
        <img
          key={f.id}
          className="floater"
          src={browserIcon}
          alt=""
          style={{
            left: f.x,
            top: f.y,
            width: ICON_SIZE,
            height: ICON_SIZE,
          }}
        />
      ))}

      <div className="home-center">
        <button className="start-btn" onClick={() => setStarted(true)}>
          开始检测
        </button>
      </div>

      <RepoBadge />
    </div>
  );
}

export default App;
