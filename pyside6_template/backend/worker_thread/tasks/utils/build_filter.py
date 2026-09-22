from typing import Sequence, Any, TypeAlias

from pydantic import BaseModel
from sqlalchemy import select, func, or_, case, ColumnElement
from sqlalchemy.orm import InstrumentedAttribute, Session
from sqlalchemy.ext.asyncio import AsyncSession

from backend.models.base import TableBase

SQLExpr: TypeAlias = ( # python typing is ASSS
    InstrumentedAttribute[Any] | ColumnElement[Any]
)

def build_exact_str(query: BaseModel, data: type[TableBase]):
    filters = []

    for field, value in query: 
        col_attr = getattr(data, field, None)
        if col_attr is None:
            continue
        if value is not None and isinstance(value, str):
            filters.append(col_attr == value)

    return filters

def build_search_fields(
        search_string: str, search_fields: set[str],
        data: type[TableBase]
    ):
    s = f"%{search_string}%"
    return or_(
        *(
            getattr(data, col).ilike(s)
            for col in search_fields
        )
    )

def query_to_pydantic(
    db: Session,
    *columns: SQLExpr,
    where: (
        SQLExpr | Sequence[SQLExpr] | None
    ) = None,
    group_by: (
        SQLExpr | Sequence[SQLExpr] | None
    ) = None,
    order_by: (
        SQLExpr | Sequence[SQLExpr] | None
    ) = None,
    limit: int | None = None,
    return_type: type[BaseModel] | type[dict] = dict,
):
    def unpack_parameter(stmt_extend, obj: SQLExpr | Sequence[SQLExpr]):
        if isinstance(obj, Sequence):
            return stmt_extend(*obj)
        return stmt_extend(obj)

    stmt = select(*columns)

    if where is not None:
        stmt = unpack_parameter(stmt.where, where)

    if group_by is not None:
        stmt = unpack_parameter(stmt.group_by, group_by)

    if order_by is not None:
        stmt = unpack_parameter(stmt.order_by, order_by)

    if limit is not None:
        stmt = stmt.limit(limit)

    result = db.execute(stmt)

    rows = result.mappings().all()

    if return_type is dict:
        return [dict(row) for row in rows]

    return [
        return_type(**row)
        for row in rows
    ]

'''
explore overload with generic for better typing
from __future__ import annotations

from collections.abc import Sequence
from typing import Any, TypeVar, overload

from pydantic import BaseModel
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select, func
from sqlalchemy.sql.elements import ColumnElement
from sqlalchemy.orm import InstrumentedAttribute


SQLExpr = InstrumentedAttribute[Any] | ColumnElement[Any]

T = TypeVar("T", bound=BaseModel)

@overload
async def query_to_pydantic(
    db: AsyncSession,
    *columns: SQLExpr,
    group_by: SQLExpr | Sequence[SQLExpr] | None = None,
    order_by: SQLExpr | Sequence[SQLExpr] | None = None,
    return_type: type[T],
) -> list[T]: ...


@overload
async def query_to_pydantic(
    db: AsyncSession,
    *columns: SQLExpr,
    group_by: SQLExpr | Sequence[SQLExpr] | None = None,
    order_by: SQLExpr | Sequence[SQLExpr] | None = None,
    return_type: type[dict] = dict,
) -> list[dict]: ...
'''