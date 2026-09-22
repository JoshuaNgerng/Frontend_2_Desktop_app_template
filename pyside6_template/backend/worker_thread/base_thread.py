from typing import Any
from PySide6.QtCore import QObject, QRunnable, Signal

class WorkerSignals(QObject):
    progress = Signal(str, str)
    error = Signal(str, dict)
    finished = Signal(str, str)

class WorkerBase(QRunnable):
    def __init_subclass__(cls, **kwargs):
        super().__init_subclass__(**kwargs)
        required_method = "run"
        if required_method not in cls.__dict__:
            raise TypeError(
                f"{cls.__name__} must define '{required_method}'"
            )

    def __init__(
            self, request_id: str,
            req_params: dict[str, Any] | None = None, app_context: Any = None
    ):
        super().__init__()
        self.request_id = request_id
        self.req_params = req_params if req_params else {}
        self.app_context = app_context
        self.progression_stage = 'start'
        self.signals = WorkerSignals()
        self._running = True

    def __repr__(self) -> str:
        return f'{self.__class__.__name__}({self.request_id}): params: {self.req_params}, app_ctx: {self.app_context}'

    def stop(self):
        self._running = False

    def emit_progress(self, stage: str):
        self.signals.progress.emit(self.request_id, stage)

    def emit_error(self, stage: str, message: str = '', data: Any = None):
        self.signals.error.emit(
            self.request_id, {
                'stage': stage,
                'message': message,
                'data': data
            })

    def emit_finished(self, data: Any | None = None):
        self.signals.finished.emit(self.request_id, data or {})

    def run(self):
        """
        Entry point called by QThread.
        Never pass any GUI objects in app_context nor any not thread safe objects from main thread
        Subclasses MUST override this.
        """
        raise NotImplementedError("Subclasses must implement run()")
