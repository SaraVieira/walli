import { useEffect } from "react";
import { useSettingsStore } from "../../store/settings";
import { ipc } from "../../shared/ipc";
import IntervalPicker from "./components/IntervalPicker";
import SourceToggles from "./components/SourceToggles";
import ApiKeyInput from "./components/ApiKeyInput";
import CollectionsEditor from "./components/CollectionsEditor";


export default function SettingsPage() {
  const { settings, collections, refresh, patch } = useSettingsStore();
  useEffect(() => {
    refresh();
  }, [refresh]);
  if (!settings) return <div className="p-6 text-muted">Loading…</div>;
  return (
    <div className="space-y-6 p-6">
      <section>
        <h2 className="mb-2 text-sm font-semibold">Rotation interval</h2>
        <IntervalPicker
          value={settings.interval_seconds}
          onChange={(s) => patch({ interval_seconds: s })}
        />
      </section>

      <section>
        <h2 className="mb-2 text-sm font-semibold">Display</h2>
        <label className="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            checked={settings.per_display_mode}
            onChange={(e) => patch({ per_display_mode: e.target.checked })}
          />
          Different wallpaper per display
        </label>
      </section>

      <section>
        <h2 className="mb-2 text-sm font-semibold">Sources</h2>
        <SourceToggles settings={settings} onPatch={patch} />
      </section>

      <section className="space-y-2">
        <h2 className="text-sm font-semibold">API keys</h2>
        <ApiKeyInput
          source="unsplash"
          label="Unsplash"
          isSet={settings.unsplash_key_set}
          onChanged={refresh}
        />
      </section>

      <section>
        <h2 className="mb-2 text-sm font-semibold">Collections</h2>
        <CollectionsEditor collections={collections} onChanged={refresh} />
      </section>

      <section>
        <label className="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            checked={settings.login_at_startup}
            onChange={async (e) => {
              await ipc.setLoginAtStartup(e.target.checked);
              refresh();
            }}
          />
          Open at login
        </label>
      </section>
    </div>
  );
}
