from functools import lru_cache
import json
from typing import Any, Callable

from PySide6.QtCore import QObject, Slot, QThreadPool, Signal
from PySide6.QtWidgets import QFileDialog

from backend.worker_thread.base_thread import WorkerBase
from backend.core.database import get_db

class BackendBridge(QObject):
    db_session = get_db()
    progress = Signal(str, str)
    error = Signal(str, dict)
    finished = Signal(str, str)

    def __init__(self, parent=None, view=None):
        super().__init__(parent=parent)
        self.view = view
        self.routes = self.get_route_mapping()
        self.pool = QThreadPool.globalInstance()
        self.active_tasks = set()
        # store active threads to prevent premature garbage collection from python

    @Slot(str, str, str)
    def request(
        self, request_id: str,
        path: str, payload_json: str
    ):

        try:
            payload = json.loads(payload_json) if payload_json else {}

            WorkerClass, app_context_func = self.routes.get(path, (None, None))

            if not WorkerClass:
                raise Exception(f"No route for {path}")

            app_context = app_context_func() if app_context_func else None

            worker = WorkerClass(request_id, req_params=payload, app_context=app_context)
            print(f'debug running worker: {worker}')
            self.run_worker(worker)

        except Exception as e:
            self.error.emit(request_id, 'Internal App Error')

    # if backend have dependencies
    def start_services(self): pass

    def shutdown(self):
        print("Shutting down backend...")

        for task in self.active_tasks:
            task.stop()

        self.active_tasks.clear()

        print("ThreadPool will clear remaining tasks automatically")

    def run_worker(self, task: WorkerBase):
        self.active_tasks.add(task)

        # connect signals
        task.signals.progress.connect(self.progress)
        task.signals.error.connect(self.error)
        task.signals.finished.connect(self.finished)
        task.signals.finished.connect(lambda _: self.active_tasks.discard(task))

        self.pool.start(task)

        return task

    def _get_file(self, desc: str = '', ext: str = '*.*'):
        file_path, _ = QFileDialog.getOpenFileName(
            self.view,
            "Select a file",
            "",
            f"{desc} ({ext})"
        )
        return file_path
    
    def get_route_mapping(self)-> dict[
        str,
        tuple[type[WorkerBase], Callable[[], Any] | None]
    ]:
        return {
            'get_example': (ExamplePydanticInfo, None),
            'post_example_upload': (
                DataIngestion,
                lambda : self._get_file('Excel/CSV Files', '*.xlsx *.xls *.csv')
            )
        }