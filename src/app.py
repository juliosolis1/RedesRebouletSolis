import sys
import socket
from PyQt6.QtWidgets import (
    QApplication, QMainWindow, QWidget, QVBoxLayout, QHBoxLayout,
    QLabel, QPushButton, QComboBox, QLineEdit, QTextEdit, QFrame,
    QGridLayout, QGroupBox
)
from PyQt6.QtCore import Qt, QTimer
from PyQt6.QtGui import QFont, QColor

class VideocallApp(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("RedesCall - Videollamada UDP & TCP/IP")
        self.resize(1000, 650)
        
        # Estado básico de sockets (Para que tu compañero lo complete)
        self.udp_socket = None
        
        self.init_ui()
        self.apply_styles()

    def init_ui(self):
        # Contenedor Principal
        central_widget = QWidget()
        self.setCentralWidget(central_widget)
        main_layout = QVBoxLayout(central_widget)
        main_layout.setContentsMargins(15, 15, 15, 15)
        main_layout.setSpacing(12)

        # --- HEADER ---
        header_frame = QFrame()
        header_frame.setObjectName("HeaderFrame")
        header_layout = QHBoxLayout(header_frame)
        header_layout.setContentsMargins(15, 10, 15, 10)

        title_label = QLabel("RedesCall  [UDP & TCP/IP]")
        title_label.setFont(QFont("Segoe UI", 14, QFont.Weight.Bold))
        title_label.setObjectName("HeaderTitle")

        db_status_label = QLabel("● BD Conectada")
        db_status_label.setObjectName("DbStatus")

        header_layout.addWidget(title_label)
        header_layout.addStretch()
        header_layout.addWidget(db_status_label)
        main_layout.addWidget(header_frame)

        # --- CUERPO PRINCIPAL (Split 2 Columnas) ---
        body_layout = QHBoxLayout()
        body_layout.setSpacing(15)

        # 1. SECCIÓN DE VIDEO (Izquierda)
        video_container = QVBoxLayout()
        video_grid = QGridLayout()
        video_grid.setSpacing(10)

        # Pantalla Remota
        self.remote_video_card = QFrame()
        self.remote_video_card.setObjectName("VideoCard")
        remote_layout = QVBoxLayout(self.remote_video_card)
        self.remote_label = QLabel("Esperando flujo de video UDP (Remoto)...")
        self.remote_label.setAlignment(Qt.AlignmentFlag.AlignCenter)
        self.remote_label.setObjectName("VideoText")
        remote_layout.addWidget(self.remote_label)

        # Pantalla Local
        self.local_video_card = QFrame()
        self.local_video_card.setObjectName("VideoCardLocal")
        local_layout = QVBoxLayout(self.local_video_card)
        self.local_label = QLabel("Cámara Local Activa")
        self.local_label.setAlignment(Qt.AlignmentFlag.AlignCenter)
        self.local_label.setObjectName("VideoTextLocal")
        local_layout.addWidget(self.local_label)

        video_grid.addWidget(self.remote_video_card, 0, 0)
        video_grid.addWidget(self.local_video_card, 0, 1)
        video_container.addLayout(video_grid)

        # Controles de Llamada
        controls_layout = QHBoxLayout()
        controls_layout.setSpacing(10)

        self.btn_mic = QPushButton("Mute Mic")
        self.btn_mic.setObjectName("BtnControl")
        self.btn_cam = QPushButton("Apagar Cam")
        self.btn_cam.setObjectName("BtnControl")
        self.btn_hangup = QPushButton("Colgar")
        self.btn_hangup.setObjectName("BtnDanger")

        controls_layout.addStretch()
        controls_layout.addWidget(self.btn_mic)
        controls_layout.addWidget(self.btn_hangup)
        controls_layout.addWidget(self.btn_cam)
        controls_layout.addStretch()
        video_container.addLayout(controls_layout)

        body_layout.addLayout(video_container, stretch=2)

        # 2. PANEL DE CONTROL Y RED (Derecha)
        panel_frame = QFrame()
        panel_frame.setObjectName("ControlPanel")
        panel_layout = QVBoxLayout(panel_frame)
        panel_layout.setContentsMargins(15, 15, 15, 15)
        panel_layout.setSpacing(10)

        panel_title = QLabel("Panel de Red UDP")
        panel_title.setFont(QFont("Segoe UI", 11, QFont.Weight.Bold))
        panel_layout.addWidget(panel_title)

        # Seleccionar Modo
        panel_layout.addWidget(QLabel("Modo de Transmisión:"))
        self.combo_mode = QComboBox()
        self.combo_mode.addItems(["Client-to-Server", "Unicast (P2P)", "Broadcast (Red Local)"])
        panel_layout.addWidget(self.combo_mode)

        # IP Destino
        panel_layout.addWidget(QLabel("IP Destino / Servidor:"))
        self.input_ip = QLineEdit("127.0.0.1")
        panel_layout.addWidget(self.input_ip)

        # Puerto UDP
        panel_layout.addWidget(QLabel("Puerto UDP:"))
        self.input_port = QLineEdit("5005")
        panel_layout.addWidget(self.input_port)

        # Botón Conectar
        self.btn_connect = QPushButton("Iniciar Transmisión")
        self.btn_connect.setObjectName("BtnPrimary")
        self.btn_connect.clicked.connect(self.log_start_call)
        panel_layout.addWidget(self.btn_connect)

        # Terminal / Consola de Logs
        panel_layout.addWidget(QLabel("Estado de Sockets / Paquetes:"))
        self.log_box = QTextEdit()
        self.log_box.setReadOnly(True)
        self.log_box.setObjectName("LogBox")
        self.log_box.append("[INIT] Interfaz PyQt6 iniciada.")
        self.log_box.append("[DB] Tablas SQLite/PostgreSQL detectadas.")
        panel_layout.addWidget(self.log_box)

        body_layout.addWidget(panel_frame, stretch=1)
        main_layout.addLayout(body_layout)

    def apply_styles(self):
        # Estilos QSS usando la paleta de Amarillo Pastel y tonos grises/cálidos
        qss = """
        QWidget {
            background-color: #FFFDF7;
            font-family: 'Segoe UI', sans-serif;
            color: #2B2D42;
        }

        #HeaderFrame {
            background-color: #FFFFFF;
            border-bottom: 2px solid #EAE3D2;
            border-radius: 8px;
        }

        #HeaderTitle {
            color: #2B2D42;
        }

        #DbStatus {
            color: #2A9D8F;
            font-weight: bold;
            font-size: 11px;
        }

        #VideoCard {
            background-color: #FAFAFA;
            border: 2px solid #EAE3D2;
            border-radius: 10px;
            min-height: 300px;
        }

        #VideoCardLocal {
            background-color: #F4F6F8;
            border: 2px solid #F4A261;
            border-radius: 10px;
            min-height: 300px;
        }

        #VideoText {
            color: #8D99AE;
            font-size: 12px;
        }

        #VideoTextLocal {
            color: #2B2D42;
            font-size: 12px;
            font-weight: bold;
        }

        #ControlPanel {
            background-color: #FFFFFF;
            border: 2px solid #EAE3D2;
            border-radius: 10px;
        }

        QLineEdit, QComboBox {
            background-color: #FFFDF7;
            border: 1.5px solid #EAE3D2;
            border-radius: 6px;
            padding: 6px;
            font-size: 12px;
        }

        QLineEdit:focus, QComboBox:focus {
            border: 1.5px solid #F4A261;
        }

        #BtnPrimary {
            background-color: #FFF3CD;
            color: #664D03;
            border: 1px solid #FFEBAA;
            font-weight: bold;
            padding: 10px;
            border-radius: 6px;
        }

        #BtnPrimary:hover {
            background-color: #FFE69C;
        }

        #BtnControl {
            background-color: #FFFFFF;
            border: 1.5px solid #EAE3D2;
            padding: 8px 15px;
            border-radius: 6px;
        }

        #BtnControl:hover {
            background-color: #FFF3CD;
        }

        #BtnDanger {
            background-color: #E76F51;
            color: white;
            font-weight: bold;
            border: none;
            padding: 8px 20px;
            border-radius: 6px;
        }

        #BtnDanger:hover {
            background-color: #D9534F;
        }

        #LogBox {
            background-color: #1E1E1E;
            color: #9CDCFE;
            font-family: 'Consolas', monospace;
            font-size: 11px;
            border-radius: 6px;
        }
        """
        self.setStyleSheet(qss)

    def log_start_call(self):
        mode = self.combo_mode.currentText()
        ip = self.input_ip.text()
        port = self.input_port.text()
        self.log_box.append(f"[SOCKET] Iniciando {mode} a {ip}:{port} via UDP...")

if __name__ == '__main__':
    app = QApplication(sys.argv)
    window = VideocallApp()
    window.show()
    sys.exit(app.exec())