from typing import Generator, Optional, AsyncGenerator 
from functools import lru_cache
from contextlib import asynccontextmanager
from sqlalchemy.ext.asyncio import (
    create_async_engine,
    async_sessionmaker,
    AsyncSession,
)
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker, Session

from contextlib import contextmanager
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker, scoped_session

from backend.core.file_storage import AppPaths

class DatabaseManager:

    def __init__(self, database_url: str):

        self.engine = create_engine(
            database_url,
            pool_pre_ping=True,
            pool_size=5,
            # poolclass=StaticPool,
            max_overflow=10,
            # connect_args={"check_same_thread": False}
        )

        # THREAD-SAFE session factory
        self.SessionLocal = scoped_session(
            sessionmaker(
                bind=self.engine,
                autocommit=False,
                autoflush=False,
                expire_on_commit=False,
            )
        )

    @contextmanager
    def session_scope(self):
        """
        Safe usage:
        with db.session_scope() as s:
            ...
        """
        session = self.SessionLocal()

        try:
            yield session
            # session.commit()

        except Exception:
            session.rollback()
            raise

        finally:
            session.close()

    def dispose(self):
        self.SessionLocal.remove()
        self.engine.dispose()

    def init_db(self):
        import backend.models
        from backend.models.base import Base
        Base.metadata.create_all(bind=self.engine)


class AsyncDatabaseManager:
    def __init__(
        self,
        database_url: str,
        pool_size: int = 5,
        max_overflow: int = 10,
        echo: bool = False,
        pool_pre_ping: bool = True,
    ):
        self.engine = create_async_engine(
            database_url,
            pool_size=pool_size,
            # poolclass=StaticPool,
            max_overflow=max_overflow,
            echo=echo,
            pool_pre_ping=pool_pre_ping,
            # connect_args={"check_same_thread": False}
        )

        self.AsyncSessionLocal = async_sessionmaker(
            bind=self.engine,
            expire_on_commit=False,
            autoflush=False,
            autocommit=False,
            class_=AsyncSession,
        )

    # -------------------------
    # FastAPI dependency
    # -------------------------
    async def get_db(self) -> AsyncGenerator[AsyncSession, None]:
        """
        Usage:
        db: AsyncSession = Depends(db_manager.get_db)
        """
        db: Optional[AsyncSession] = None
        try:
            db = self.AsyncSessionLocal()
            yield db
        finally:
            if db:
                await db.close()

    # -------------------------
    # Manual session
    # -------------------------
    def get_session(self) -> AsyncSession:
        return self.AsyncSessionLocal()

    # -------------------------
    # Transaction context manager (like your sync version)
    # -------------------------
    @asynccontextmanager
    async def session_scope(self) -> AsyncGenerator[AsyncSession, None]:
        """
        Usage:
        async with db_manager.session_scope() as db:
            ...
        """
        db: AsyncSession = self.AsyncSessionLocal()

        try:
            yield db
            await db.commit()
        except Exception:
            await db.rollback()
            raise
        finally:
            await db.close()

    # -------------------------
    # Cleanup
    # -------------------------
    async def dispose(self):
        await self.engine.dispose()

    async def init_db(self):
        import backend.models  # ensure all models are registered
        from backend.models.base import Base

        async with self.engine.begin() as conn:
            await conn.run_sync(Base.metadata.create_all)

@lru_cache
def get_db():
    print(AppPaths.db_path())
    return DatabaseManager(f"sqlite:///{AppPaths.db_path()}")

@lru_cache
def get_async_db():
    return AsyncDatabaseManager(f"sqlite:///{AppPaths.db_path()}")