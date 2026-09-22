import subprocess
from pathlib import Path

# =========================================
# CONFIG
# =========================================

# directory -> alias (None = keep full path)
INCLUDE_DIRS = {
    "frontend/build": '',
    "frontend/static": "static",
    "config": None
}

QRC_FILE = "resources.qrc"
OUTPUT_PY = "resources_rc.py"

# =========================================
# GENERATE QRC
# =========================================

lines = [
    "<RCC>",
    '  <qresource prefix="/">'
]

for folder, alias in INCLUDE_DIRS.items():
    folder_path = Path(folder)

    if not folder_path.exists():
        print(f"Skipping missing folder: {folder}")
        continue

    for file in folder_path.rglob("*"):
        if not file.is_file():
            continue

        if alias is not None:
            # map into alias namespace
            relative = file.relative_to(folder_path).as_posix()
            qrc_path = f"{alias}/{relative}"
            if qrc_path[0] == '/':
                qrc_path = qrc_path[1:]
            lines.append(f'    <file alias="{qrc_path}">{file.as_posix()}</file>')
        else:
            # keep full path inside qrc
            lines.append(f"    <file>{file.as_posix()}</file>")

lines += [
    "  </qresource>",
    "</RCC>"
]

Path(QRC_FILE).write_text("\n".join(lines), encoding="utf-8")

print(f"Generated {QRC_FILE}")

# =========================================
# COMPILE QRC
# =========================================

subprocess.run([
    "pyside6-rcc",
    QRC_FILE,
    "-o",
    OUTPUT_PY
], check=True)

print(f"Generated {OUTPUT_PY}")