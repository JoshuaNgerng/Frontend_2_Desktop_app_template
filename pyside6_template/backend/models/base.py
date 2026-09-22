from datetime import datetime

from sqlalchemy import DateTime, Integer, MetaData
from sqlalchemy.orm import mapped_column, Mapped
from sqlalchemy.ext.declarative import declarative_base, declarative_base

convention = {
    "ix": "ix_%(table_name)s_%(column_0_name)s",
    "uq": "uq_%(table_name)s_%(column_0_name)s",
    "ck": "ck_%(table_name)s_%(constraint_name)s",
    "fk": "fk_%(table_name)s_%(column_0_name)s_%(referred_table_name)s",
    "pk": "pk_%(table_name)s",
}

metadata = MetaData(naming_convention=convention)

Base = declarative_base(metadata=metadata)

class TableBase(Base):
    """Base class for all database tables with common columns."""
    __abstract__ = True
    
    id: Mapped[int] = mapped_column(Integer, primary_key=True, index=True, autoincrement=True)
    created_at: Mapped[datetime] = mapped_column(DateTime(timezone=True), default=datetime.now, nullable=False)
    updated_at: Mapped[datetime] = mapped_column(DateTime(timezone=True), default=datetime.now, onupdate=datetime.now, nullable=False)

    # Use this method to convert model instance to a dict (for API responses)
    def to_dict(self) -> dict:
        """Convert model to dictionary."""
        result = {}
        for column in self.__table__.columns:
            value = getattr(self, column.name)
            if isinstance(value, datetime):
                result[column.name] = value.isoformat()
            else:
                result[column.name] = value
        return result