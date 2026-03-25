// cosmic-applet-capslock
// Caps Lock indicator for the COSMIC panel.
//
// API reference: https://pop-os.github.io/libcosmic-book/panel-applets.html

use cosmic::{
    app::Core,
    iced::{
        self,
        platform_specific::shell::commands::popup::{destroy_popup, get_popup},
        widget::column,
        Alignment, Limits, Subscription,
    },
    widget::{divider, text},
    Application, Element,
};
use iced::window;
use std::time::Duration;

mod keyboard;

const APP_ID: &str = "com.system76.CosmicAppletCapslock";

// ── Messages ───────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    CapsLockChanged(bool),
}

// ── State ──────────────────────────────────────────────────────────────────
pub struct CapsLockApplet {
    core: Core,
    popup: Option<window::Id>,
    caps_active: bool,
    icon_name: String,
}

impl Application for CapsLockApplet {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: ()) -> (Self, cosmic::Task<cosmic::Action<Message>>) {
        let applet = Self {
            core,
            popup: None,
            caps_active: keyboard::query_caps_lock(),
            icon_name: String::from("input-keyboard-symbolic"),
        };
        (applet, cosmic::Task::none())
    }

    // ── Transparent panel background ─────────────────────────────────────────
    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }

    // ── Panel button ─────────────────────────────────────────────────────────
    fn view(&self) -> Element<Message> {
        self.core
            .applet
            .icon_button(&self.icon_name)
            .on_press_down(Message::TogglePopup)
            .into()
    }

    // ── Popup contents ───────────────────────────────────────────────────────
    fn view_window(&self, id: window::Id) -> Element<Message> {
        if matches!(self.popup, Some(p) if p == id) {
            let status = if self.caps_active { "On" } else { "Off" };
            let description = if self.caps_active {
                "ALL CAPS typing is active"
            } else {
                "Normal typing mode"
            };

            let content = column![
                text::heading("Caps Lock"),
                divider::horizontal::default(),
                text::title1(status),
                text::body(description),
            ]
            .align_x(Alignment::Center)
            .spacing(8)
            .padding(16);

            self.core
                .applet
                .popup_container(content)
                .max_height(160.)
                .max_width(220.)
                .into()
        } else {
            text::body("").into()
        }
    }

    // ── Update ───────────────────────────────────────────────────────────────
    fn update(&mut self, message: Message) -> cosmic::Task<cosmic::Action<Message>> {
        match message {
            Message::TogglePopup => {
                if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = window::Id::unique();
                    self.popup.replace(new_id);

                    let mut popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        Some((220, 160)),
                        None,
                        None,
                    );

                    popup_settings.positioner.size_limits = Limits::NONE
                        .min_width(180.0)
                        .min_height(100.0)
                        .max_height(200.0)
                        .max_width(260.0);

                    get_popup(popup_settings)
                }
            }

            Message::CapsLockChanged(state) => {
                self.caps_active = state;
                cosmic::Task::none()
            }
        }
    }

    // ── Subscription: poll every 250 ms ──────────────────────────────────────
    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_millis(250))
            .map(|_| Message::CapsLockChanged(keyboard::query_caps_lock()))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cosmic::applet::run::<CapsLockApplet>(())
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}
