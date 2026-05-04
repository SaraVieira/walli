import { useWallpaperStore } from "../../../store/wallpaper";

export default function Controls() {
  const { paused, next, setPaused } = useWallpaperStore();
  return (
    <div className="flex gap-2 text-xs">
      <button
        onClick={next}
        className="flex-1 rounded bg-neutral-800 px-3 py-2 hover:bg-neutral-700"
      >
        Next
      </button>
      <button
        onClick={() => setPaused(!paused)}
        className="flex-1 rounded bg-neutral-800 px-3 py-2 hover:bg-neutral-700"
      >
        {paused ? "Resume" : "Pause"}
      </button>
    </div>
  );
}
