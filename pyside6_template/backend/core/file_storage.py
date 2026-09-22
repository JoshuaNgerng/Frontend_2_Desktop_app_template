from PySide6.QtCore import QStandardPaths
from pathlib import Path
import json


class TempStorage:

    def __init__(self, dirname='my_app'):
        temp_dir = QStandardPaths.writableLocation(QStandardPaths.StandardLocation.TempLocation)
        self.base_dir = Path(temp_dir) / dirname

    def save_text(self, filename, content):
        return self.__save_file(filename, content)

    def save_json(self, filename, data):
        return self.__save_file(filename, data, method=json.dump)

    def save_binary(self, filename, data):
        return self.__save_file(filename, data, binary=True)

    def __save_file(self, filename, data, binary=False, method=None):
        path = self.base_dir / filename
        mode = "w" if not binary else "wb"
        encoding = "utf-8" if not binary else None
        with open(path, mode=mode, encoding=encoding) as f:
            if not method:
                f.write(data)
            else:
                method(data, f)
        return path

    def get_path(self, filename):
        return str(self.base_dir / filename)


class AppPaths:
    @staticmethod
    def db_path():
        base = QStandardPaths.writableLocation(
            QStandardPaths.StandardLocation.AppDataLocation
        )
        return Path(base) / "app.db"