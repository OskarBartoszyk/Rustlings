#!/usr/bin/env python3
"""
Usage:
  python3 check.py          check exercises in order, stop at the first unfinished one
  python3 check.py 7        check only exercise 07
  python3 check.py list     show progress
  python3 check.py --solutions   (maintainer) verify the reference solutions
"""
import pathlib, subprocess, sys

root = pathlib.Path(__file__).resolve().parent
folder = "solutions" if "--solutions" in sys.argv else "exercises"
args = [a for a in sys.argv[1:] if not a.startswith("--")]
files = sorted((root / folder).glob("*.rs"))
(root / "build").mkdir(exist_ok=True)


def run(f):
    exe = root / "build" / f.stem
    c = subprocess.run(["rustc", "--edition", "2021", "-A", "warnings", "-o", str(exe), str(f)],
                       capture_output=True, text=True)
    if c.returncode != 0:
        return False, "COMPILE ERROR (read the message - Rust's compiler is a great teacher):\n" + c.stderr
    r = subprocess.run([str(exe)], capture_output=True, text=True)
    if r.returncode != 0:
        return False, "RUNTIME FAILURE (an assert or todo!() failed):\n" + r.stdout + r.stderr
    return True, r.stdout


if args and args[0] == "list":
    for f in files:
        ok, _ = run(f)
        print(("✅" if ok else "⬜"), f.name)
    sys.exit()

if args:
    n = int(args[0])
    files = [f for f in files if f.name.startswith(f"{n:02d}_")]
    if not files:
        sys.exit("no such exercise")

for f in files:
    ok, msg = run(f)
    if ok:
        print("✅", f.name)
        if args:
            print(msg)
    else:
        print(f"\n❌ {folder}/{f.name}\n\n{msg}")
        print(f"👉 Open {folder}/{f.name}, read the lesson comments, solve the TASK, then run again.")
        sys.exit(1)
print("\n🎉 All done!")
