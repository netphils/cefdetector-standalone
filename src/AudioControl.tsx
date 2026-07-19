import { useEffect, useRef, useState } from "react";
import bgm from "./assets/bgm.opus";
import speakerOn from "./assets/speaker-2-svgrepo-com.svg";
import speakerOff from "./assets/speaker-cross-svgrepo-com.svg";
import "./AudioControl.css";

type Props = {
  playing: boolean;
};

export default function AudioControl({ playing }: Props) {
  const audioRef = useRef<HTMLAudioElement>(null);
  const [muted, setMuted] = useState(false);

  useEffect(() => {
    const audio = audioRef.current;
    if (!audio) return;
    if (playing) {
      audio.currentTime = 0;
      audio.play().catch(() => {});
    } else {
      audio.pause();
    }
  }, [playing]);

  useEffect(() => {
    const audio = audioRef.current;
    if (audio) audio.muted = muted;
  }, [muted]);

  return (
    <>
      <audio ref={audioRef} src={bgm} loop preload="auto" />
      <button
        className="audio-btn"
        onClick={() => setMuted((m) => !m)}
        aria-label={muted ? "取消静音" : "静音"}
      >
        <img src={muted ? speakerOff : speakerOn} alt="" />
      </button>
    </>
  );
}
