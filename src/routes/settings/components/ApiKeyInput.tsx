import { useState } from "react";
import { ipc } from "../../../shared/ipc";
import type { SourceKind } from "../../../shared/types";

export default function ApiKeyInput(props: {
  source: SourceKind;
  label: string;
  isSet: boolean;
  onChanged: () => void;
}) {
  const [val, setVal] = useState("");
  return (
    <div className="flex items-center gap-2">
      <span className="w-24 text-sm">{props.label}</span>
      <input
        type="password"
        value={val}
        onChange={(e) => setVal(e.target.value)}
        placeholder={props.isSet ? "•••• stored" : "paste key"}
        className="flex-1 rounded bg-neutral-800 px-2 py-1 text-sm"
      />
      <button
        onClick={async () => {
          if (val) {
            await ipc.setApiKey(props.source, val);
            setVal("");
            props.onChanged();
          }
        }}
        className="rounded bg-accent px-2 py-1 text-xs text-white"
      >
        Save
      </button>
      {props.isSet && (
        <button
          onClick={async () => {
            await ipc.clearApiKey(props.source);
            props.onChanged();
          }}
          className="text-xs text-muted hover:text-fg"
        >
          Clear
        </button>
      )}
    </div>
  );
}
