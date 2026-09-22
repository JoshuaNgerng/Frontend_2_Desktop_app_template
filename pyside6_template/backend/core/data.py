from backend.core.utils.load_yaml import load_yaml_resource
from backend.core.utils.load_json import load_json_resource
from PySide6.QtCore import QUrl

COL_MAP = load_yaml_resource("config/example.yaml")
DATA_SEEDER = load_json_resource("config/example.json")
