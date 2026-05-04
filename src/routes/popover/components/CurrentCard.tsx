import { convertFileSrc } from "@tauri-apps/api/core";
import { useWallpaperStore } from "../../../store/wallpaper";
import { ipc } from "../../../shared/ipc";

export default function CurrentCard() {
  const { current } = useWallpaperStore();
  if (!current)
    return <div className="aspect-video w-full rounded-md bg-neutral-900" />;
  const src = convertFileSrc(current.file_path);
  return (
    <div className="space-y-2">
      <img
        src={src}
        alt=""
        className="aspect-video w-full rounded-md object-cover"
      />
      <div className="flex items-center justify-between text-xs">
        <span className="text-muted">
          {current.photographer ?? "Unknown"} · {current.source}
        </span>
        <button
          onClick={() => ipc.toggleFavorite(current.id)}
          className="rounded px-2 py-0.5 text-accent hover:bg-neutral-800"
        >
          ★
        </button>
      </div>
    </div>
  );
}
