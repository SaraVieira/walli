import { convertFileSrc } from "@tauri-apps/api/core";
import { useWallpaperStore } from "../../../store/wallpaper";
import { ipc } from "../../../shared/ipc";
import { Button } from "@/components/ui/button";
import { FlipCard } from "@/components/animate-ui/components/community/flip-card";

export default function CurrentCard() {
  const { current, paused, loading, next, setPaused } = useWallpaperStore();
  if (!current)
    return <div className="aspect-video w-full rounded-md bg-neutral-900" />;
  const src = convertFileSrc(current.file_path);

  const data = {
    name: current.title ?? "",
    image: src,
    bio: `${current.photographer ?? "Unknown"} · ${current.source}`,
  };
  return (
    <div className="space-y-2">
      <FlipCard data={data}>
        {current.title && <div className="text-xs">{current.title}</div>}
        <div className="text-xs text-muted-foreground">
          {current.photographer ?? "Unknown"} · {current.source}
        </div>
        <Button onClick={next} disabled={loading}>
          {loading ? "Working…" : "Next"}
        </Button>
        <Button onClick={() => setPaused(!paused)}>
          {paused ? "Resume" : "Pause"}
        </Button>
        <button
          onClick={() => ipc.toggleFavorite(current.id)}
          className="rounded px-2 py-0.5 text-accent hover:bg-neutral-800"
          aria-label={current.is_favorite ? "Unfavorite" : "Favorite"}
        >
          {current.is_favorite ? "★" : "☆"}
        </button>
      </FlipCard>
    </div>
  );
}
