import type { Settings } from "../../../shared/types";

const SOURCES: { key: keyof Settings; label: string }[] = [
  { key: "source_unsplash_enabled", label: "Unsplash" },
  { key: "source_bing_enabled", label: "Bing daily" },
];

export default function SourceToggles(props: {
  settings: Settings;
  onPatch: (p: Partial<Settings>) => void;
}) {
  return (
    <div className="space-y-1 text-sm">
      {SOURCES.map(({ key, label }) => (
        <label key={key} className="flex items-center gap-2">
          <input
            type="checkbox"
            checked={!!props.settings[key]}
            onChange={(e) =>
              props.onPatch({ [key]: e.target.checked } as Partial<Settings>)
            }
          />
          {label}
        </label>
      ))}
    </div>
  );
}
