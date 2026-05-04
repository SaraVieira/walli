import { useEffect } from "react";
import { useSettingsStore } from "../../../store/settings";
import { ipc } from "../../../shared/ipc";

export default function CollectionSwitcher() {
  const { collections, settings, refresh } = useSettingsStore();
  useEffect(() => {
    refresh();
  }, [refresh]);
  if (!collections.length) return null;
  const active = settings?.active_collection_id ?? null;
  return (
    <select
      className="w-full rounded bg-neutral-800 px-2 py-1 text-xs"
      value={active ?? ""}
      onChange={async (e) => {
        const id = Number(e.target.value);
        await ipc.setActiveCollection(id);
        refresh();
      }}
    >
      <option value="" disabled>
        Choose collection…
      </option>
      {collections.map((c) => (
        <option key={c.id} value={c.id}>
          {c.name}
        </option>
      ))}
    </select>
  );
}
