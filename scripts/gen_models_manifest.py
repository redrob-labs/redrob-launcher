"""Regenerate assets/models/models.json from Redrob Film's pinned catalogue.

Usage: python3 scripts/gen_models_manifest.py <redrob-film checkout> assets/models/models.json
"""
import json
import re
import sys

film = sys.argv[1]
out = sys.argv[2]

speech = open(f"{film}/crates/speech/src/models.rs", encoding="utf-8").read()
tts = open(f"{film}/crates/tts/src/catalog.rs", encoding="utf-8").read()

models = []
for block in re.findall(r"ModelInfo \{(.*?)\n    \},", speech, re.S):
    get = lambda k: re.search(rf'{k}: "([^"]*)"', block).group(1)
    files = []
    for repo, rev, name, sha, size in re.findall(r'hf!\(\s*"([^"]+)",\s*"([0-9a-f]{40})",\s*"([^"]+)",\s*"([0-9a-f]{64})",\s*([0-9_]+)\s*\)', block):
        files.append({"name": name, "url": f"https://huggingface.co/openai/{repo}/resolve/{rev}/{name}", "sha256": sha, "size": int(size.replace("_", ""))})
    models.append({
        "id": get("id"),
        "app": "redrob-film",
        "name": get("name"),
        "description": get("description"),
        "license": "MIT (OpenAI Whisper weights); Hugging Face conversion: Apache-2.0",
        "source": get("source"),
        "files": files,
    })

tts_files = []
base = re.search(r'concat!\("(https://huggingface.co/hexgrad/Kokoro-82M/resolve/[0-9a-f]{40}/)"', tts).group(1)
for name, path, sha, size in re.findall(r'hf!\("([^"]+)", "([^"]+)", "([0-9a-f]{64})", ([0-9_]+)\)', tts):
    tts_files.append({"name": name, "url": base + path, "sha256": sha, "size": int(size.replace("_", ""))})
m = re.search(r'name: "cmudict.dict",\s*url: "([^"]+)",\s*sha256: "([0-9a-f]{64})",\s*size: ([0-9_]+)', tts)
tts_files.insert(2, {"name": "cmudict.dict", "url": m.group(1), "sha256": m.group(2), "size": int(m.group(3).replace("_", ""))})
models.append({
    "id": "kokoro-82m",
    "app": "redrob-film",
    "name": "Natural voices (Kokoro-82M)",
    "description": "Nine US English narration voices and the CMUdict pronunciation dictionary.",
    "license": "Apache-2.0 (Kokoro-82M by hexgrad); CMUdict: BSD-style (Carnegie Mellon University)",
    "source": "https://huggingface.co/hexgrad/Kokoro-82M",
    "files": tts_files,
})

doc = {
    "$comment": "Models the launcher can download for each Redrob app, into the folder that app reads them from. Every file is pinned to a revision and its SHA-256. Generated from each app's own catalogue (Redrob Film: crates/speech/src/models.rs and crates/tts/src/catalog.rs); keep in step with it.",
    "version": 1,
    "models": models,
}
open(out, "w", encoding="utf-8").write(json.dumps(doc, indent=2) + "\n")
for x in models:
    print(x["id"], len(x["files"]), sum(f["size"] for f in x["files"]))
