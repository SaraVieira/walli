import { useState } from "react";
import { ipc } from "../../../shared/ipc";
import type { SourceKind } from "../../../shared/types";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";

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
      <Input
        type="password"
        value={val}
        onChange={(e) => setVal(e.target.value)}
        placeholder={props.isSet ? "•••• stored" : "paste key"}
      />
      <Button
        onClick={async () => {
          if (val) {
            await ipc.setApiKey(props.source, val);
            setVal("");
            props.onChanged();
          }
        }}
      >
        Save
      </Button>
      {props.isSet && (
        <Button
          onClick={async () => {
            await ipc.clearApiKey(props.source);
            props.onChanged();
          }}
          variant="destructive"
        >
          Clear
        </Button>
      )}
    </div>
  );
}
