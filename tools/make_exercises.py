#!/usr/bin/env python3
"""Generate exercises/ from solutions/ by blanking out the `// [[ ... // ]]` regions."""
import pathlib, re

root = pathlib.Path(__file__).resolve().parent.parent
out = root / "exercises"
out.mkdir(exist_ok=True)

for src in sorted((root / "solutions").glob("*.rs")):
    res, skipping = [], False
    for line in src.read_text().splitlines():
        s = line.strip()
        if s.startswith("// [[") and not skipping:
            skipping = True
            ind = line[: len(line) - len(line.lstrip())]
            if s.startswith("// [[-"):
                res.append(f"{ind}// TODO: write your code here")
            else:
                res.append(f"{ind}todo!() // <-- replace this with your code")
        elif s == "// ]]" and skipping:
            skipping = False
        elif not skipping:
            res.append(line)
    (out / src.name).write_text("\n".join(res) + "\n")
print("generated", len(list(out.glob('*.rs'))), "exercises")
