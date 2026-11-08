use eframe::egui;
use crate::network::NetworkManager;

pub struct RedesApp {
    network: NetworkManager,
    port_input: String,
    target_ip: String,
    selected_mode: usize,
    log_output: String,
    is_muted: bool,
    is_cam_off: bool,
}

impl Default for RedesApp {
    fn default() -> Self {
        Self {
            network: NetworkManager::new(),
            port_input: "5005".to_string(),
            target_ip: "127.0.0.1".to_string(),
            selected_mode: 0,
            log_output: "[INIT] Interfaz nativa Rust iniciada.\n[NET] Sin conexión establecida (Socket Inactivo).".to_string(),
            is_muted: false,
            is_cam_off: false,
        }
    }
}

impl eframe::App for RedesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- CONFIGURACIÓN DE ESTÉTICA: AMARILLO PASTEL Y BOTONES CLAROS ---
        let mut style = (*ctx.style()).clone();
        
        // Forzar tema visual base claro para evitar que los widgets hereden tonos oscuros
        style.visuals = egui::Visuals::light();

        let bg_pastel = egui::Color32::from_rgb(255, 253, 235);    // Fondo general amarillo pastel muy suave
        let panel_bg = egui::Color32::from_rgb(255, 250, 220);     // Fondo interno de cuadros y contenedores
        let text_dark = egui::Color32::from_rgb(40, 40, 40);       // Texto principal oscuro para alta nitidez
        let border_yellow = egui::Color32::from_rgb(220, 200, 130); // Bordes en tono cálido
        let btn_inactive = egui::Color32::from_rgb(250, 230, 150); // Botón amarillo pastel visible y claro
        let btn_hover = egui::Color32::from_rgb(235, 210, 120);    // Color al pasar el cursor
        let input_bg = egui::Color32::from_rgb(255, 255, 255);     // Fondo blanco limpio para inputs y consola

        // Aplicar colores globales
        style.visuals.window_fill = bg_pastel;
        style.visuals.panel_fill = bg_pastel;
        style.visuals.override_text_color = Some(text_dark);

        // Estilos limpios para los botones (Inactivos, Hover y Activos)
        style.visuals.widgets.inactive.bg_fill = btn_inactive;
        style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, border_yellow);
        style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text_dark);

        style.visuals.widgets.hovered.bg_fill = btn_hover;
        style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.5, text_dark);
        style.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.5, text_dark);

        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(210, 180, 90);
        style.visuals.widgets.active.fg_stroke = egui::Stroke::new(1.5, text_dark);

        // Contenedores y paneles
        style.visuals.widgets.noninteractive.bg_fill = panel_bg;
        style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, border_yellow);
        style.visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, text_dark);

        // Estilos específicos para campos de texto (Inputs) y consola
        style.visuals.extreme_bg_color = input_bg;

        ctx.set_style(style);
        // -----------------------------------------------------------

        // Barra superior con título y estado de conexión real
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.heading("RedesCall [UDP & TCP/IP]");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.network.is_active() {
                        ui.colored_label(egui::Color32::from_rgb(0, 130, 0), "● Socket Activo");
                    } else {
                        ui.colored_label(egui::Color32::from_rgb(170, 30, 30), "● Sin Conexión Establecida");
                    }
                });
            });
            ui.add_space(5.0);
        });

        // Panel lateral derecho (Controles de red y consola de logs)
        egui::SidePanel::right("right_panel").default_width(300.0).show(ctx, |ui| {
            ui.add_space(10.0);
            ui.heading("Panel de Red UDP");
            ui.separator();

            ui.label("Modo de Transmisión:");
            let modes = ["Client-to-Server", "Unicast (P2P)", "Broadcast"];
            egui::ComboBox::from_label("")
                .selected_text(modes[self.selected_mode])
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected_mode, 0, modes[0]);
                    ui.selectable_value(&mut self.selected_mode, 1, modes[1]);
                    ui.selectable_value(&mut self.selected_mode, 2, modes[2]);
                });

            ui.add_space(10.0);
            ui.label("IP Destino / Servidor:");
            ui.text_edit_singleline(&mut self.target_ip);

            ui.add_space(10.0);
            ui.label("Puerto UDP:");
            ui.text_edit_singleline(&mut self.port_input);

            ui.add_space(15.0);
            if ui.button("Iniciar Transmisión").clicked() {
                match self.port_input.parse::<u16>() {
                    Ok(port) => {
                        match self.network.bind_socket(port) {
                            Ok(msg) => {
                                self.log_output.push_str(&format!("\n{}", msg));
                            }
                            Err(e) => {
                                self.log_output.push_str(&format!("\n[ERROR] Fallo al vincular puerto {}: {}", port, e));
                            }
                        }
                    }
                    Err(_) => {
                        self.log_output.push_str("\n[ERROR] El puerto debe ser numérico.");
                    }
                }
            }

            ui.add_space(20.0);
            ui.label("Estado de Sockets / Paquetes:");
            
            // Consola de logs real
            egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.log_output)
                        .code_editor()
                        .desired_width(f32::INFINITY)
                );
            });
        });

        // Panel Central (Cámaras y botones inferiores)
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                // Columna Izquierda: Vista Remota
                columns[0].vertical_centered(|ui| {
                    ui.add_space(10.0);
                    let size = ui.available_size();
                    let box_size = egui::vec2(size.x - 10.0, size.y - 65.0);
                    
                    ui.group(|ui| {
                        ui.set_min_size(box_size);
                        ui.vertical_centered(|ui| {
                            ui.add_space(box_size.y / 2.0 - 20.0);
                            if self.network.is_active() {
                                ui.label("Recibiendo flujo de video remoto...");
                            } else {
                                ui.colored_label(egui::Color32::DARK_GRAY, "Esperando flujo de video UDP (Remoto)...\n(Sin conexión establecida)");
                            }
                        });
                    });
                });

                // Columna Derecha: Vista Local
                columns[1].vertical_centered(|ui| {
                    ui.add_space(10.0);
                    let size = ui.available_size();
                    let box_size = egui::vec2(size.x - 10.0, size.y - 65.0);

                    ui.group(|ui| {
                        ui.set_min_size(box_size);
                        ui.vertical_centered(|ui| {
                            ui.add_space(box_size.y / 2.0 - 20.0);
                            if self.is_cam_off {
                                ui.colored_label(egui::Color32::from_rgb(170, 30, 30), "Cámara Local Apagada");
                            } else {
                                ui.label("Cámara Local Activa\n(Listo para transmitir)");
                            }
                        });
                    });
                });
            });

            ui.add_space(10.0);
            
            // Botones inferiores de control de llamada
            ui.horizontal(|ui| {
                let mute_label = if self.is_muted { "Activar Mic" } else { "Mute Mic" };
                if ui.button(mute_label).clicked() {
                    self.is_muted = !self.is_muted;
                }

                if ui.button("Colgar").clicked() {
                    self.log_output.push_str("\n[NET] Llamada finalizada. Socket cerrado.");
                    self.network = NetworkManager::new();
                }

                let cam_label = if self.is_cam_off { "Encender Cam" } else { "Apagar Cam" };
                if ui.button(cam_label).clicked() {
                    self.is_cam_off = !self.is_cam_off;
                }
            });
        });
    }
}