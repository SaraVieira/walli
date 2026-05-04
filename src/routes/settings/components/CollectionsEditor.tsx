import { useState } from "react";
import { ipc } from "../../../shared/ipc";
import type { Collection } from "../../../shared/types";

export default function CollectionsEditor(props: {
  collections: Collection[];
  onChanged: () => void;
}) {
  const [newName, setNewName] = useState("");
  const [newTags, setNewTags] = useState("");
  return (
    <div className="space-y-2">
      <ul className="space-y-2">
        {props.collections.map((c) => (
          <CollectionRow
            key={c.id}
            collection={c}
            onChanged={props.onChanged}
          />
        ))}
      </ul>
      <div className="flex gap-2">
        <input
          value={newName}
          onChange={(e) => setNewName(e.target.value)}
          placeholder="Collection name"
          className="flex-1 rounded bg-neutral-800 px-2 py-1 text-sm"
        />
        <input
          value={newTags}
          onChange={(e) => setNewTags(e.target.value)}
          placeholder="tag1, tag2"
          className="flex-1 rounded bg-neutral-800 px-2 py-1 text-sm"
        />
        <button
          onClick={async () => {
            if (!newName.trim()) return;
            const tags = newTags
              .split(",")
              .map((t) => t.trim())
              .filter(Boolean);
            await ipc.createCollection(newName.trim(), tags);
            setNewName("");
            setNewTags("");
            props.onChanged();
          }}
          className="rounded bg-accent px-3 py-1 text-sm text-white"
        >
          Add
        </button>
      </div>
    </div>
  );
}

function CollectionRow({
  collection,
  onChanged,
}: {
  collection: Collection;
  onChanged: () => void;
}) {
  const [name, setName] = useState(collection.name);
  const [tags, setTags] = useState(collection.tags.join(", "));
  return (
    <li className="flex items-center gap-2">
      <input
        value={name}
        onChange={(e) => setName(e.target.value)}
        className="w-32 rounded bg-neutral-800 px-2 py-1 text-sm"
      />
      <input
        value={tags}
        onChange={(e) => setTags(e.target.value)}
        className="flex-1 rounded bg-neutral-800 px-2 py-1 text-sm"
      />
      <button
        onClick={async () => {
          const t = tags
            .split(",")
            .map((x) => x.trim())
            .filter(Boolean);
          await ipc.updateCollection(collection.id, name, t);
          onChanged();
        }}
        className="text-xs text-muted hover:text-fg"
      >
        Save
      </button>
      <button
        onClick={async () => {
          await ipc.deleteCollection(collection.id);
          onChanged();
        }}
        className="text-xs text-red-400"
      >
        Delete
      </button>
    </li>
  );
}
