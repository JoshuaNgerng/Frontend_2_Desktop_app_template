import sys

from PySide6.QtCore import QUrl
from PySide6.QtWidgets import QApplication, QFileDialog
from PySide6.QtWebEngineWidgets import QWebEngineView
from PySide6.QtWebEngineCore import QWebEngineSettings
from PySide6.QtWebChannel import QWebChannel
from PySide6.QtWebEngineCore import QWebEnginePage
from PySide6.QtCore import QDateTime

from backend.core.database import get_db
from backend.bridge import BackendBridge

import resources_rc

class ConsolePage(QWebEnginePage):

    def javaScriptConsoleMessage(
        self,
        level,
        message,
        lineNumber,
        sourceID
    ):
        """
        Captures console.log / warn / error from WebEngine
        """
        timestamp = QDateTime.currentDateTime().toString("HH:mm:ss")

        level_map = {
            QWebEnginePage.JavaScriptConsoleMessageLevel.InfoMessageLevel: "INFO",
            QWebEnginePage.JavaScriptConsoleMessageLevel.WarningMessageLevel: "WARN",
            QWebEnginePage.JavaScriptConsoleMessageLevel.ErrorMessageLevel: "ERROR",
        }

        level_str = level_map.get(level, "LOG")

        print(
            f"[{timestamp}] [JS-{level_str}] "
            f"{message} (line:{lineNumber}, source:{sourceID})"
        )

class DesktopApp:

    def __init__(self, file_path: str, app: QApplication):
        self.app = app
        self.view = QWebEngineView()

        self.backend = BackendBridge()
        self.backend.start_services()
        self.channel = QWebChannel()
        self.page = ConsolePage(self.view)
        self.view.setPage(self.page)

        self.setup_webchannel()
        self.configure_browser()
        self.load_frontend(file_path)

        self.view.resize(1200, 800)

        self.init_db()

        self.view.page().runJavaScript("""
        if (!window.QWebChannelScriptLoaded) {
            const script = document.createElement('script');
            script.src = 'qrc:///qtwebchannel/qwebchannel.js';
            script.onload = () => {
                console.log("QWebChannel loaded");
            };
            document.head.appendChild(script);
        }
        """)
        self.app.aboutToQuit.connect(
            self.shutdown
        )

    def setup_webchannel(self):
        self.bridge = BackendBridge(view=self.view)
        self.channel.registerObject("bridge", self.bridge)
        self.view.page().setWebChannel(self.channel)

    def configure_browser(self):
        settings = self.view.settings()
        # Allow JS to open local files
        settings.setAttribute(
            QWebEngineSettings.WebAttribute.LocalContentCanAccessFileUrls,
            True
        )
        # # Allow JS/local page to call remote APIs
        # settings.setAttribute(
        #     QWebEngineSettings.WebAttribute.LocalContentCanAccessRemoteUrls,
        #     True
        # )
        # General JS support
        settings.setAttribute(
            QWebEngineSettings.WebAttribute.JavascriptEnabled,
            True
        )
        # Local storage / cache
        settings.setAttribute(
            QWebEngineSettings.WebAttribute.LocalStorageEnabled,
            True
        )

    def load_frontend(self, file_path: str):
        self.view.setUrl(QUrl(f"qrc:/{file_path}"))

    def init_db(self):
        get_db().init_db()
        # example seeder.seed()

    def shutdown(self):

        print("Shutting down desktop app...")

        try:
            self.backend.shutdown()

        except Exception as e:
            print("Shutdown error:", e)


def main():

    app = QApplication(sys.argv)

    desktop = DesktopApp('index.html', app)

    desktop.view.show()

    exit_code = app.exec()

    sys.exit(exit_code)



if __name__ == "__main__":
    main()