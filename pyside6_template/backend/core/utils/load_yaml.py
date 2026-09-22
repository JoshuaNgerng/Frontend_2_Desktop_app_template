from pathlib import Path
from PySide6.QtCore import QFile, QIODevice

import yaml
import resources_rc

def load_yaml_resource(path: str):
    file = QFile(f':/{path.lstrip('/')}')

    try:
        if not file.open(QIODevice.OpenModeFlag.ReadOnly | QIODevice.OpenModeFlag.Text):
            raise IOError(file.errorString())

        content = bytes(file.readAll().data())
        return yaml.safe_load(content.decode("utf-8"))

    finally:
        file.close()


def build_reverse_mapping(
    mapping: dict[str, list[str]],
) -> dict[str, str]:
    """
    Converts:

    {
        "vehicle_make": ["Vehicle_Make", "Make"]
    }

    into:

    {
        "Vehicle_Make": "vehicle_make",
        "Make": "vehicle_make"
    }
    """

    reverse = {}

    for db_col, aliases in mapping.items():

        for alias in aliases:

            if alias in reverse:
                raise ValueError(
                    f"Duplicate alias detected: {alias}"
                )

            reverse[alias] = db_col

    return reverse

def load_mapping_local(
        path: Path | str
) -> dict[str, list[str]]:

    with open(path, "r", encoding="utf-8") as f:
        mapping = yaml.safe_load(f)

    return mapping