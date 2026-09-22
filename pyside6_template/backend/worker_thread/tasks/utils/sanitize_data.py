from decimal import Decimal
from datetime import datetime, date
from functools import lru_cache
import re
import polars as pl
from typing import TYPE_CHECKING
from sqlalchemy import types as satypes

if TYPE_CHECKING:
    from backend.models.base import TableBase

_SQLACLCHEMY_TYPE_MAP = [
    (satypes.Integer, int),
    (satypes.BIGINT, int),
    (satypes.Numeric, float),
    (satypes.String, str),
    (satypes.DateTime, datetime),
    (satypes.Boolean, bool),
]

_PYTHON_TO_PL = {
    int: pl.Int64,
    float: pl.Float64,
    str: pl.Utf8,
    bool: pl.Boolean,
    Decimal: pl.Decimal(18, 4),

    datetime: pl.Datetime("us"),
    date: pl.Date,
}


def apply_column_mapping(
    df: pl.DataFrame,
    mapping: dict[str, list[str]]
) -> pl.DataFrame:
    '''
    ensure columns are mapped to expected columns
    if duplicated exist priority will be:
        - the intended name in the mapping
        - the name first in order of the mapping list
        - if none found throw ValueError
    all duplicated column will be dropped
    column will be snake case to fit db 
    '''

    rename_map = {}
    cols_to_drop = []

    for canonical, aliases in mapping.items():

        # priority:
        # 1. canonical name itself
        # 2. aliases in listed order
        candidates = [canonical] + aliases

        matches = [c for c in candidates if c in df.columns]

        if not matches:
            raise ValueError(
                f"Missing required column for '{canonical}'"
            )

        # canonical always wins if present
        if canonical in matches:
            chosen = canonical
        else:
            chosen = matches[0]

        # rename chosen column if needed
        if chosen != canonical:
            rename_map[chosen] = canonical

        # all other matched columns become redundant
        redundant = [c for c in matches if c != chosen]
        cols_to_drop.extend(redundant)

    # apply rename
    df = df.rename(rename_map)

    # remove duplicates/redundant aliases
    if cols_to_drop:
        df = df.drop(cols_to_drop)

    # ensure snake case for db
    df = df.rename({col: to_snake(col) for col in df.columns})
    return df

def validate_data(
    df: pl.DataFrame,
    schema: dict[str, tuple[type, bool]]
) -> pl.DataFrame:
    # ---- cast columns ----
    cast_exprs = []

    for col, (py_type, nullable) in schema.items():

        if col not in df.columns:
            raise ValueError(f"Missing required column: {col}")

        if py_type not in _PYTHON_TO_PL:
            raise TypeError(
                f"Unsupported python type for '{col}': {py_type}"
            )

        pl_type = _PYTHON_TO_PL[py_type]

        cast_exprs.append(
            pl.col(col).cast(pl_type, strict=False)
        )

    try:
        df = df.with_columns(cast_exprs)

    except Exception as e:
        raise ValueError(
            f"Failed schema type enforcement. "
            f"Could not convert one or more columns."
        ) from e

    # ---- validate nullability ----
    violations = []

    for col, (_, nullable) in schema.items():

        if not nullable:
            null_count = df.select(
                pl.col(col).is_null().sum()
            ).item()

            if null_count > 0:
                violations.append(
                    f"{col} contains {null_count} null values"
                )

    if violations:
        raise ValueError(
            "Schema nullability violations:\n"
            + "\n".join(violations)
        )

    # Replace NaN / empty values with None
    df = df.with_columns([
        pl.when(pl.col(c).is_nan())
        .then(None)
        .otherwise(pl.col(c))
        .alias(c)
        for c in df.columns
        if df.schema[c].is_float()
    ])

    return df

def to_snake(name: str) -> str:
    name = name.strip()
    name = re.sub(r"(?<!^)(?=[A-Z])", "_", name)  # camelCase → snake_case
    name = re.sub(r"\s+", "_", name)              # spaces → underscore
    name = re.sub(r"[^a-zA-Z0-9_]", "_", name)    # remove special chars
    return name.lower()

def map_sqlalchemy_type(col_type):
    for sa_type, py_type in _SQLACLCHEMY_TYPE_MAP:
        if isinstance(col_type, sa_type):
            return py_type
    return str  # fallback

def get_table_schema(
    model: type[TableBase], ignore_key: set[str] | None = None
):
    schema : dict[str, tuple[type, bool]] = {}
    ignore_key = set() if not ignore_key else ignore_key
    for column in model.__table__.columns:
        if str(column.name) in ignore_key: continue
        py_type = map_sqlalchemy_type(column.type)
        schema[str(column.name)] = (py_type, column.nullable)

    return schema

def add_batch_id(df: pl.DataFrame, batch_id: str):
    '''
    manually add a new column with batch id to df
    '''
    return df.with_columns(
        pl.lit(batch_id).alias("upload_batch")
    )

def add_filename(df: pl.DataFrame, filename: str):
    '''
    manually add a new column with batch id to df
    '''
    return df.with_columns(
        pl.lit(filename).alias("filename")
    )
