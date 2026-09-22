from pathlib import Path
from typing import Any
from PySide6.QtCore import QFile, QIODevice

import json
import resources_rc

def load_json_resource(path: str):
    file = QFile(f':/{path.lstrip('/')}')

    try:
        if not file.open(QIODevice.OpenModeFlag.ReadOnly):
            raise IOError(file.errorString())

        content = file.readAll().toStdString()
        return json.loads(content)

    except json.JSONDecodeError as e:
        raise IOError(f"{path} contains invalid JSON: {e}")

    except Exception as e:
        raise IOError(f"{path} cannot be loaded: {e}")

    finally:
        file.close()

def load_json_local(path: Path | str) -> Any:
    path = Path(path)
    if not (path.exists() and path.is_file()):
        raise IOError(f"{path} don't exist or is not a file")
    try:
        with open(path, 'r') as f:
            return json.load(f)

    except json.JSONDecodeError as e:
        raise IOError(f"{path} contains invalid JSON: {e}")

    except Exception as e:
        raise IOError(f"{path} cannot be loaded: {e}")