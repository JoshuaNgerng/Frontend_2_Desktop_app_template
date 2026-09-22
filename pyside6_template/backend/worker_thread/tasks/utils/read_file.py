from pathlib import Path
import polars as pl

def read_file(file_path: Path):
    check_file(file_path)
    if file_path.suffix == ".csv":
        df = read_csv(file_path)

    elif file_path.suffix in [".xlsx", ".xls"]:
        df = read_xlsx(file_path)
    else: 
        raise LoadDatasetError(f"Unsupported file type: {file_path.suffix}")
    return df

def read_csv(file_path: Path):
    encodings = ["utf8", "utf8-lossy", "latin1"]

    last_error = None

    for enc in encodings:
        try:
            return pl.read_csv(file_path, encoding=enc)

        except UnicodeDecodeError as e:
            last_error = e
            continue

        except Exception as e:
            # non-encoding parsing error → fail fast
            raise LoadDatasetError(f"CSV parsing failed: {e}") from e

    raise LoadDatasetError(f"All encodings failed in CSV parsing. Last error: {last_error}")

def read_xlsx(file_path: Path):
    try:
        return pl.read_excel(file_path)
    except Exception as e:
        raise LoadDatasetError(f"Excel parsing failed: {e}")

def check_file(file_path: Path):
    try:
        if not file_path.exists():
            raise FileNotFoundError(f"File not found: {file_path}")

        if not file_path.is_file():
            raise IsADirectoryError(f"Expected file but got directory: {file_path}")

    except (FileNotFoundError, PermissionError, IsADirectoryError) as e:
        raise LoadDatasetError(f"Dataset access error: {e}") from e

class LoadDatasetError(Exception):
    def __init__(self, message: str) -> None:
        super().__init__(message)