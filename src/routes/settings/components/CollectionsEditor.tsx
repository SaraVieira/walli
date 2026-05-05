import { useState } from "react";
import { ipc } from "../../../shared/ipc";
import type { Collection } from "../../../shared/types";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";

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
      <div className="flex gap-2 mt-4">
        <Input
          value={newName}
          onChange={(e) => setNewName(e.target.value)}
          placeholder="Collection name"
          className="w-32"
        />
        <Input
          value={newTags}
          onChange={(e) => setNewTags(e.target.value)}
          placeholder="tag1, tag2"
          className="flex-1"
        />
        <Button
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
        >
          Add
        </Button>
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
      <Input
        value={name}
        onChange={(e) => setName(e.target.value)}
        className="w-32"
      />
      <Input
        value={tags}
        onChange={(e) => setTags(e.target.value)}
        className="flex-1"
      />
      <Button
        onClick={async () => {
          const t = tags
            .split(",")
            .map((x) => x.trim())
            .filter(Boolean);
          await ipc.updateCollection(collection.id, name, t);
          onChanged();
        }}
      >
        Save
      </Button>
      <Button
        onClick={async () => {
          await ipc.deleteCollection(collection.id);
          onChanged();
        }}
        variant={"destructive"}
      >
        Delete
      </Button>
    </li>
  );
}
