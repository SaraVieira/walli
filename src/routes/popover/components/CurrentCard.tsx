import { convertFileSrc } from "@tauri-apps/api/core";
import { useWallpaperStore } from "../../../store/wallpaper";
import { Button } from "@/components/ui/button";
import { FlipCard } from "@/components/animate-ui/components/community/flip-card";
import { ChevronRight, Loader, Pause, Play } from "lucide-react";

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
    <div className="space-y-2 -mt-2">
      <FlipCard data={data}>
        <div className="flex flex-col items-start justify-between h-full w-full">
          <div>
            {current.title && <div className="text-xs">{current.title}</div>}
            <div className="text-xs text-muted-foreground">
              {current.photographer ?? "Unknown"} · {current.source}
            </div>
          </div>
          <div className="flex justify-between items-center w-full">
            <Button onClick={next} disabled={loading} variant={"outline"}>
              {loading ? <Loader /> : <ChevronRight />}
            </Button>
            <Button variant={"outline"} onClick={() => setPaused(!paused)}>
              {paused ? <Play /> : <Pause />}
            </Button>
          </div>
        </div>
      </FlipCard>
    </div>
  );
}
