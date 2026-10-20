"""Decide whether a pull request needs the heavy CI jobs.

Prints `heavy=true` or `heavy=false` (append it to $GITHUB_OUTPUT). A PR is light when every changed
file is documentation, or when the diff only moves this repository's own version strings: the
`version` of a [package]/[workspace.package] table in our Cargo.toml files, the matching
`version = ` lines of our own crates in Cargo.lock, and VERSION-style files. Anything else is heavy,
and so is anything this script cannot read.

Usage: python3 scripts/ci_changes.py <base-sha> <head-sha>
"""

import fnmatch
import re
import subprocess
import sys
import tomllib

DOCS = ["*.md", "docs/*", "LICENSE*", "CHANGELOG*", ".github/*.md"]
VERSION_FILES = ["VERSION", "REDROB_VERSION", "*-version.txt"]
VERSION_LINE = re.compile(r'^\s*version\s*=\s*"[^"]*"\s*$')
HUNK = re.compile(r"^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@")


def git(*args: str) -> str:
    return subprocess.run(["git", *args], check=True, capture_output=True, text=True).stdout


def show(rev: str, path: str) -> list[str]:
    try:
        return git("show", f"{rev}:{path}").splitlines()
    except subprocess.CalledProcessError:
        return []


def matches(path: str, patterns: list[str]) -> bool:
    name = path.rsplit("/", 1)[-1]
    return any(fnmatch.fnmatch(path, p) or fnmatch.fnmatch(name, p) for p in patterns)


def own_crates(rev: str) -> set[str]:
    names = set()
    for path in git("ls-tree", "-r", "--name-only", rev).splitlines():
        if path.rsplit("/", 1)[-1] == "Cargo.toml":
            try:
                pkg = tomllib.loads("\n".join(show(rev, path))).get("package", {})
            except tomllib.TOMLDecodeError:
                continue
            if isinstance(pkg.get("name"), str):
                names.add(pkg["name"])
    return names


def changed_lines(base: str, head: str, path: str):
    """Yield (side, line_number) for every changed line; side is 'old' or 'new'."""
    old = new = 0
    for line in git("diff", "-U0", base, head, "--", path).splitlines():
        m = HUNK.match(line)
        if m:
            old, new = int(m.group(1)), int(m.group(3))
        elif line.startswith(("---", "+++")):
            continue
        elif line.startswith("-"):
            yield "old", old
            old += 1
        elif line.startswith("+"):
            yield "new", new
            new += 1


def section_of(lines: list[str], n: int) -> str:
    for line in reversed(lines[: n - 1]):
        if line.strip().startswith("["):
            return line.strip()
    return ""


def lock_name_of(lines: list[str], n: int) -> str:
    for line in reversed(lines[: n - 1]):
        m = re.match(r'^name = "([^"]+)"$', line)
        if m:
            return m.group(1)
        if line.strip() == "[[package]]":
            break
    return ""


def version_only(base: str, head: str, path: str, crates: set[str]) -> bool:
    name = path.rsplit("/", 1)[-1]
    if matches(path, VERSION_FILES):
        return True
    if name not in ("Cargo.toml", "Cargo.lock", "package.json"):
        return False
    files = {"old": show(base, path), "new": show(head, path)}
    seen = False
    for side, n in changed_lines(base, head, path):
        lines = files[side]
        if not 0 < n <= len(lines):
            return False
        text = lines[n - 1]
        seen = True
        if name == "package.json":
            if not re.match(r'^ {2}"version"\s*:\s*"[^"]*",?\s*$', text):
                return False
        elif not VERSION_LINE.match(text):
            return False
        elif name == "Cargo.toml":
            if section_of(lines, n) not in ("[package]", "[workspace.package]"):
                return False
        elif lock_name_of(lines, n) not in crates:
            return False
    return seen


def main() -> None:
    base, head = sys.argv[1], sys.argv[2]
    base = git("merge-base", base, head).strip()
    files = [f for f in git("diff", "--name-only", base, head).splitlines() if f]
    crates = own_crates(base) | own_crates(head)
    light = bool(files) and all(
        matches(f, DOCS) or version_only(base, head, f, crates) for f in files
    )
    for f in files:
        print(f"changed: {f}", file=sys.stderr)
    print(f"heavy={'false' if light else 'true'}")


if __name__ == "__main__":
    try:
        main()
    except Exception as exc:  # unreadable diff: run everything
        print(f"ci_changes: {exc}", file=sys.stderr)
        print("heavy=true")
