use eframe::egui::{self, Align, Color32, FontId, Layout, RichText, Stroke, Vec2};

const INK: Color32 = Color32::from_rgb(25, 26, 28);
const MUTED: Color32 = Color32::from_rgb(111, 113, 117);
const LINE: Color32 = Color32::from_rgb(229, 230, 232);
const PAPER: Color32 = Color32::from_rgb(249, 249, 247);
const WHITE: Color32 = Color32::WHITE;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1040.0, 760.0])
            .with_min_inner_size([760.0, 580.0])
            .with_title("BashLens"),
        ..Default::default()
    };
    eframe::run_native(
        "BashLens",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            Ok(Box::new(BashLens::default()))
        }),
    )
}

#[derive(Default)]
struct BashLens {
    command: String,
    analyzed: Analysis,
    edited: bool,
    notice: Option<String>,
    page: Page,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum Page {
    #[default]
    Review,
    Settings,
    About,
}

#[derive(Default, Clone)]
struct Analysis {
    actions: Vec<Action>,
    shell: &'static str,
    multi_line: bool,
}

#[derive(Clone)]
struct Action {
    title: &'static str,
    detail: String,
    level: Level,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Level {
    Review,
    High,
    Unknown,
}

impl BashLens {
    fn analyze(&mut self) {
        self.analyzed = analyze(&self.command);
        self.edited = false;
        self.notice = None;
    }

    fn topbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("◉").size(20.0).color(INK).strong());
            ui.label(RichText::new("BashLens").size(17.0).strong().color(INK));
            ui.add_space(8.0);
            ui.label(
                RichText::new("LOCAL COMMAND REVIEW")
                    .size(10.0)
                    .color(MUTED)
                    .strong(),
            );
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let status = if self.page == Page::Settings {
                    "MONITORING OFF"
                } else {
                    "MANUAL REVIEW"
                };
                ui.label(RichText::new(status).size(10.0).color(MUTED).strong());
                ui.add_space(6.0);
                ui.label(RichText::new("●").size(10.0).color(INK));
            });
        });
    }

    fn sidebar(&mut self, ui: &mut egui::Ui) {
        ui.add_space(26.0);
        ui.label(RichText::new("WORKSPACE").size(10.0).strong().color(MUTED));
        ui.add_space(10.0);
        for (page, icon, title) in [
            (Page::Review, "⌕", "Review command"),
            (Page::Settings, "⚙", "Settings"),
            (Page::About, "i", "About BashLens"),
        ] {
            let selected = self.page == page;
            let button = egui::Button::new(
                RichText::new(format!("{icon}    {title}"))
                    .size(13.0)
                    .color(if selected { INK } else { MUTED })
                    ,
            )
            .fill(if selected {
                Color32::from_rgb(239, 239, 236)
            } else {
                Color32::TRANSPARENT
            })
            .stroke(Stroke::NONE)
            .corner_radius(7.0)
            .min_size(Vec2::new(ui.available_width(), 38.0));
            if ui.add(button).clicked() {
                self.page = page;
            }
        }
        ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
            ui.add_space(14.0);
            ui.label(
                RichText::new("Commands stay on this device.\nHistory is off by default.")
                    .size(11.0)
                    .color(MUTED),
            );
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);
            ui.label(
                RichText::new("BASH LENS  ·  PREVIEW")
                    .size(9.0)
                    .strong()
                    .color(MUTED),
            );
        });
    }

    fn review_page(&mut self, ui: &mut egui::Ui) {
        ui.add_space(18.0);
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("Review a command").size(25.0).strong().color(INK));
                ui.add_space(4.0);
                ui.label(RichText::new("Understand the visible actions before you use it.").size(13.0).color(MUTED));
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui.button("Paste from clipboard").clicked() {
                    match arboard::Clipboard::new().and_then(|mut c| c.get_text()) {
                        Ok(text) => { self.command = text; self.analyze(); }
                        Err(_) => self.notice = Some("Clipboard text could not be read. You can paste directly into the command box.".into()),
                    }
                }
                ui.add_space(6.0);
                if ui.button("Clear").clicked() {
                    self.command.clear(); self.analyzed = Analysis::default(); self.notice = None; self.edited = false;
                }
            });
        });
        ui.add_space(18.0);
        ui.label(
            RichText::new("COMMAND TEXT")
                .size(10.0)
                .strong()
                .color(MUTED),
        );
        ui.add_space(7.0);
        let command_response = egui::Frame::new()
            .fill(WHITE)
            .stroke(Stroke::new(1.0, LINE))
            .corner_radius(9.0)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.add_sized(
                    [ui.available_width(), 146.0],
                    egui::TextEdit::multiline(&mut self.command)
                        .font(FontId::monospace(13.0))
                        .hint_text("Paste or type a command here…")
                        .desired_rows(6)
                        .desired_width(f32::INFINITY)
                        .frame(false),
                )
            });
        if command_response.inner.changed() && !self.analyzed.actions.is_empty() {
            self.edited = true;
        }
        if self.edited {
            ui.horizontal(|ui| {
                ui.label(RichText::new("EDITED").size(9.0).strong().color(INK));
                ui.label(
                    RichText::new("This text differs from the command you first reviewed.")
                        .size(11.0)
                        .color(MUTED),
                );
            });
        }
        ui.add_space(9.0);
        ui.horizontal(|ui| {
            let can_analyze = !self.command.trim().is_empty();
            if ui
                .add_enabled(
                    can_analyze,
                    egui::Button::new(RichText::new("Analyze command").strong().color(WHITE))
                        .fill(INK)
                        .corner_radius(7.0)
                        .min_size(Vec2::new(145.0, 36.0)),
                )
                .clicked()
            {
                self.analyze();
            }
            if !self.analyzed.actions.is_empty()
                && !self.edited
                && ui.button("Copy command").clicked()
            {
                ui.ctx().copy_text(self.command.clone());
                self.notice = Some("Reviewed text copied to the clipboard.".into());
            }
        });
        if let Some(note) = &self.notice {
            ui.add_space(7.0);
            ui.label(RichText::new(note).size(11.0).color(MUTED));
        }
        ui.add_space(19.0);
        if self.command.trim().is_empty() && self.analyzed.actions.is_empty() {
            self.empty_state(ui);
        } else if self.edited {
            egui::Frame::new().fill(Color32::from_rgb(244, 244, 241)).corner_radius(9.0).inner_margin(15.0).show(ui, |ui| {
                ui.label(RichText::new("Review the updated text").size(13.0).strong().color(INK));
                ui.add_space(4.0);
                ui.label(RichText::new("The command changed after the last analysis. Analyze it again before using the notes below.").size(11.0).color(MUTED));
            });
        } else {
            self.analysis_panel(ui);
        }
    }

    fn empty_state(&self, ui: &mut egui::Ui) {
        egui::Frame::new().fill(Color32::from_rgb(244, 244, 241)).corner_radius(9.0).inner_margin(18.0).show(ui, |ui| {
            ui.label(RichText::new("A careful read, not a safety guarantee").size(14.0).strong().color(INK));
            ui.add_space(6.0);
            ui.label(RichText::new("BashLens checks command patterns locally and points out what it recognizes. It does not run commands, inspect downloaded files, or prove a command is safe.").size(12.0).color(MUTED));
        });
    }

    fn analysis_panel(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("WHAT WE NOTICED")
                    .size(10.0)
                    .strong()
                    .color(MUTED),
            );
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(
                    RichText::new(format!(
                        "{}  ·  {}",
                        self.analyzed.shell,
                        if self.analyzed.multi_line {
                            "MULTI-LINE"
                        } else {
                            "SINGLE-LINE"
                        }
                    ))
                    .size(9.0)
                    .strong()
                    .color(MUTED),
                );
            });
        });
        ui.add_space(8.0);
        if self.analyzed.actions.is_empty() {
            self.notice_panel(ui, "No listed patterns found", "This only means the local checks did not recognize a pattern. It does not mean the command is safe or fully understood.", Level::Unknown);
        } else {
            for action in &self.analyzed.actions {
                self.notice_panel(ui, action.title, &action.detail, action.level);
                ui.add_space(7.0);
            }
        }
        ui.add_space(4.0);
        ui.label(RichText::new("The analyzer sees only the text above. Aliases, variables, local programs, and remote scripts may do more than shown here.").size(11.0).color(MUTED));
    }

    fn notice_panel(&self, ui: &mut egui::Ui, title: &str, detail: &str, level: Level) {
        let tag = match level {
            Level::Review => "REVIEW",
            Level::High => "HIGH IMPACT",
            Level::Unknown => "LIMIT",
        };
        egui::Frame::new()
            .fill(WHITE)
            .stroke(Stroke::new(1.0, LINE))
            .corner_radius(8.0)
            .inner_margin(11.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(tag).size(9.0).strong().color(INK));
                    ui.add_space(5.0);
                    ui.label(RichText::new(title).size(12.0).strong().color(INK));
                });
                ui.add_space(3.0);
                ui.label(RichText::new(detail).size(11.0).color(MUTED));
            });
    }

    fn settings_page(&mut self, ui: &mut egui::Ui) {
        ui.add_space(18.0);
        ui.label(RichText::new("Settings").size(25.0).strong().color(INK));
        ui.add_space(4.0);
        ui.label(
            RichText::new("Choose how BashLens behaves on this device.")
                .size(13.0)
                .color(MUTED),
        );
        ui.add_space(20.0);
        egui::Frame::new()
            .fill(WHITE)
            .stroke(Stroke::new(1.0, LINE))
            .corner_radius(9.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("Clipboard monitoring")
                                .size(14.0)
                                .strong()
                                .color(INK),
                        );
                        ui.label(
                            RichText::new(
                                "Automatic terminal interception is not enabled in this preview.",
                            )
                            .size(11.0)
                            .color(MUTED),
                        );
                    });
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new("OFF").size(10.0).strong().color(MUTED));
                    });
                });
                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);
                for (name, summary) in [
                    (
                        "Supported destinations",
                        "Manual review only · no terminal is currently intercepted",
                    ),
                    (
                        "Command history",
                        "Off · command text is not saved by BashLens",
                    ),
                    ("Network access", "None · analysis runs locally"),
                ] {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(name).size(12.0).strong().color(INK));
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(RichText::new(summary).size(11.0).color(MUTED));
                        });
                    });
                    ui.add_space(10.0);
                }
            });
    }

    fn about_page(&self, ui: &mut egui::Ui) {
        ui.add_space(18.0);
        ui.label(
            RichText::new("A little more context")
                .size(25.0)
                .strong()
                .color(INK),
        );
        ui.add_space(4.0);
        ui.label(
            RichText::new("BashLens helps you pause and understand visible command patterns.")
                .size(13.0)
                .color(MUTED),
        );
        ui.add_space(20.0);
        for (heading, body) in [
            ("Private by design", "Command analysis runs on this device. No account, network service, command history, or diagnostic command logging is used."),
            ("A prompt, not a verdict", "Shells can interpret variables, aliases, programs, and scripts in ways that are not visible in pasted text. Treat every note as a reason to inspect."),
            ("Current preview limits", "This build supports Windows, macOS, and Linux as a native desktop interface. It provides manual clipboard review only; it does not monitor or intercept terminal pastes."),
        ] {
            egui::Frame::new().fill(WHITE).stroke(Stroke::new(1.0, LINE)).corner_radius(8.0).inner_margin(13.0).show(ui, |ui| { ui.label(RichText::new(heading).size(13.0).strong().color(INK)); ui.add_space(5.0); ui.label(RichText::new(body).size(11.5).color(MUTED)); });
            ui.add_space(8.0);
        }
    }
}

impl eframe::App for BashLens {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(PAPER))
            .show(ctx, |ui| {
                ui.add_space(11.0);
                self.topbar(ui);
                ui.add_space(11.0);
                ui.separator();
                ui.horizontal_top(|ui| {
                    ui.allocate_ui_with_layout(
                        Vec2::new(202.0, ui.available_height()),
                        Layout::top_down(Align::Min),
                        |ui| {
                            self.sidebar(ui);
                        },
                    );
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);
                    ui.allocate_ui_with_layout(
                        Vec2::new(ui.available_width(), ui.available_height()),
                        Layout::top_down(Align::Min),
                        |ui| {
                            egui::ScrollArea::vertical()
                                .auto_shrink([false, false])
                                .show(ui, |ui| match self.page {
                                    Page::Review => self.review_page(ui),
                                    Page::Settings => self.settings_page(ui),
                                    Page::About => self.about_page(ui),
                                });
                        },
                    );
                });
            });
    }
}

fn analyze(text: &str) -> Analysis {
    let s = text.to_lowercase();
    let mut actions = Vec::new();
    let mut add = |title: &'static str, detail: String, level: Level| {
        if !actions.iter().any(|a: &Action| a.title == title) {
            actions.push(Action {
                title,
                detail,
                level,
            });
        }
    };
    if contains_any(
        &s,
        &[
            "curl ",
            "curl\t",
            "wget ",
            "invoke-webrequest",
            " iwr ",
            "irm ",
            "bitsadmin",
            "start-bitstransfer",
        ],
    ) {
        add("Downloads or requests online content", "A common web request or download tool appears. BashLens does not fetch or inspect the returned content.".into(), Level::Review);
    }
    if contains_any(
        &s,
        &[
            "| sh",
            "|sh",
            "| bash",
            "|bash",
            "| zsh",
            "|zsh",
            "iex",
            "invoke-expression",
        ],
    ) && contains_any(&s, &["curl", "wget", "webrequest", "irm", "iwr"])
    {
        add("Runs content from a web request", "A download appears connected to a shell or expression runner. The content it would run is not visible here.".into(), Level::High);
    }
    if contains_any(
        &s,
        &[
            " rm ",
            "rm -",
            "rm\t",
            "rmdir",
            "remove-item",
            " del ",
            "del /",
            "erase ",
            "format ",
            "shred ",
        ],
    ) {
        add("May remove or overwrite files", "A file removal, wipe, or format pattern was found. Check the target path and any recursive or force options.".into(), Level::High);
    }
    if contains_any(&s, &["chmod", "chown", "set-acl", "icacls", "takeown"]) {
        add(
            "Changes file permissions or ownership",
            "Permission or ownership settings may change for the referenced files or folders."
                .into(),
            Level::Review,
        );
    }
    if contains_any(
        &s,
        &[
            "sudo ",
            " su ",
            "runas ",
            "start-process -verb runas",
            "-verb runas",
            "elevated",
        ],
    ) {
        add("Requests elevated privileges", "This pattern may run with administrator or root permissions, which can affect more of the system.".into(), Level::High);
    }
    if contains_any(
        &s,
        &[
            "apt install",
            "apt-get install",
            "dnf install",
            "yum install",
            "pacman -s",
            "brew install",
            "winget install",
            "choco install",
            "pip install",
            "npm install",
            "cargo install",
            "apt remove",
            "brew uninstall",
            "winget uninstall",
        ],
    ) {
        add("Installs or removes software", "A package manager command was found. Review the package name, source, and requested changes.".into(), Level::Review);
    }
    if contains_any(
        &s,
        &[
            "/etc/hosts",
            ".bashrc",
            ".zshrc",
            "profile",
            "startup",
            "currentversion\\run",
            "launchagents",
            "systemd",
            "crontab",
        ],
    ) {
        add(
            "Touches startup or configuration",
            "A common shell, startup, or system configuration location appears in the command."
                .into(),
            Level::Review,
        );
    }
    if contains_any(
        &s,
        &[
            "iptables",
            "ufw ",
            "firewall",
            "set-mppreference",
            "disable-realtimemonitoring",
            "reg add",
            "security set",
            "spctl",
            "csrutil",
        ],
    ) {
        add(
            "May change security or system settings",
            "A firewall, security, registry, or system setting pattern was found.".into(),
            Level::High,
        );
    }
    if contains_any(
        &s,
        &[
            "http://", "https://", "ssh ", "scp ", "rsync ", "nc ", "netcat", "ftp ",
        ],
    ) {
        add("Contacts an online service or remote host", "A URL or common network tool appears. The app cannot tell what data may be sent or received.".into(), Level::Review);
    }
    if contains_any(
        &s,
        &[
            " -enc ",
            "-encodedcommand",
            "base64",
            "frombase64string",
            "certutil -decode",
            "xxd -r",
        ],
    ) {
        add("Contains encoded or transformed text", "Encoded content can hide what will run. BashLens does not decode or execute payloads in this preview.".into(), Level::Unknown);
    }
    if contains_any(
        &s,
        &[
            "bash ",
            "sh ",
            "zsh ",
            "powershell",
            "cmd /c",
            "python ",
            "python3 ",
            "node ",
            "wscript",
            "cscript",
            "source ",
            " ./",
        ],
    ) {
        add("Launches another program or script", "The command starts a program or script whose full behavior may not be visible in this text.".into(), Level::Unknown);
    }
    if contains_any(&s, &["&&", "||", ";", "|", "\n"]) {
        add("Chains multiple steps", "More than one command may run in sequence or pass data between programs. Read each part before continuing.".into(), Level::Review);
    }
    drop(add);    if actions.is_empty() {
        actions.push(Action { title: "No listed pattern found", detail: "The local checks did not recognize one of their listed patterns. That is not evidence that the command is safe.".into(), level: Level::Unknown });
    }
    let shell = if s.contains("powershell") || s.contains("invoke-") || s.contains("winget") {
        "Likely PowerShell"
    } else if s.contains("cmd /c") || s.contains("reg add") {
        "Shell unclear"
    } else if s.contains("apt ")
        || s.contains("sudo ")
        || s.contains("chmod")
        || s.contains("bash")
        || s.contains("curl")
    {
        "Likely POSIX shell"
    } else {
        "Shell unclear"
    };
    Analysis {
        actions,
        shell,
        multi_line: text.lines().count() > 1,
    }
}

fn contains_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}
