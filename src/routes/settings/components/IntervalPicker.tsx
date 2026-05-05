import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

const PRESETS = [
  { label: "15 min", s: 15 * 60 },
  { label: "30 min", s: 30 * 60 },
  { label: "1 hr", s: 60 * 60 },
  { label: "3 hr", s: 3 * 60 * 60 },
  { label: "6 hr", s: 6 * 60 * 60 },
  { label: "24 hr", s: 24 * 60 * 60 },
];

export default function IntervalPicker(props: {
  value: number;
  onChange: (s: number) => void;
}) {
  const isPreset = PRESETS.some((p) => p.s === props.value);
  return (
    <div className="flex flex-wrap items-center gap-2">
      {PRESETS.map((p) => (
        <Button
          key={p.s}
          onClick={() => props.onChange(p.s)}
          variant={props.value === p.s ? "default" : "outline"}
        >
          {p.label}
        </Button>
      ))}
      <label className="flex items-center gap-1 text-xs">
        <Input
          type="number"
          min={1}
          value={isPreset ? "" : Math.round(props.value / 60)}
          onChange={(e) => props.onChange(Number(e.target.value) * 60)}
          placeholder="custom min"
          className="w-24 rounded bg-neutral-800 px-2 py-1"
        />
      </label>
    </div>
  );
}
