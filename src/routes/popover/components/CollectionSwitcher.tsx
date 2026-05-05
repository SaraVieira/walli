import { useEffect } from "react";
import { useSettingsStore } from "../../../store/settings";
import { useWallpaperStore } from "../../../store/wallpaper";
import { ipc } from "../../../shared/ipc";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

export default function CollectionSwitcher() {
  const { collections, settings, refresh } = useSettingsStore();
  useEffect(() => {
    refresh();
  }, [refresh]);
  if (!collections.length) return null;
  const active = settings?.active_collection_id ?? null;
  return (
    <Select
      defaultValue={active ? String(active) : undefined}
      onValueChange={async (value) => {
        const id = Number(value);
        useWallpaperStore.getState().beginLoading();
        await ipc.setActiveCollection(id);
        refresh();
      }}
    >
      <SelectTrigger className="w-full">
        <SelectValue placeholder="Theme" />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          {collections.map((c) => (
            <SelectItem key={c.id} value={String(c.id)}>
              {c.name}
            </SelectItem>
          ))}
        </SelectGroup>
      </SelectContent>
    </Select>
  );
}
