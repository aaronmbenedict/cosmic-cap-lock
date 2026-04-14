// cosmic-applet-capslock
// Caps Lock indicator for the COSMIC panel.

use cosmic::{
    app::Core,
    iced::{
        self,
        platform_specific::shell::commands::popup::{destroy_popup, get_popup},
        Alignment, Color, Length, Limits, Subscription,
    },
    widget::{container, divider, text},
    Application, Element,
};
use iced::window;
use std::time::Duration;

mod keyboard;

const APP_ID: &str = "com.system76.CosmicAppletCapslock";

#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    CapsLockChanged(bool),
}

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

    fn core(&self) -> &Core { &self.core }
    fn core_mut(&mut self) -> &mut Core { &mut self.core }

    fn init(core: Core, _flags: ()) -> (Self, cosmic::Task<cosmic::Action<Message>>) {
        let applet = Self {
            core,
            popup: None,
            caps_active: keyboard::query_caps_lock(),
            icon_name: String::from("input-keyboard-symbolic"),
        };
        (applet, cosmic::Task::none())
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }

    fn view(&self) -> Element<'_, Message> {
        let (icon_w, icon_h) = self.core.applet.suggested_size(true);

        let dot_color = if self.caps_active {
            cosmic::theme::active().cosmic().accent_color().into()
        } else {
            Color::TRANSPARENT
        };

        let icon: Element<_> = cosmic::widget::icon::from_name("input-keyboard-symbolic")
            .size(icon_h)
            .into();

        // Space::new() takes 0 args; width/height are chained methods
        let dot = container(cosmic::widget::Space::new().width(6))
            .style(move |_theme: &cosmic::Theme| {
                cosmic::widget::container::Style {
                    background: Some(cosmic::iced::Background::Color(dot_color)),
                    border: cosmic::iced::Border {
                        radius: 3.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
            .height(3);

        let indicator: Element<_> = cosmic::widget::column::with_children(vec![
            icon,
            container(dot)
                .width(Length::Fixed(icon_w as f32))
                .align_x(cosmic::iced::alignment::Horizontal::Center)
                .into(),
        ])
        .align_x(Alignment::Center)
        .spacing(1)
        .into();

        cosmic::widget::button::custom(indicator)
            .class(cosmic::theme::Button::AppletIcon)
            .on_press(Message::TogglePopup)
            .into()
    }

    fn view_window(&self, id: window::Id) -> Element<'_, Message> {
        if matches!(self.popup, Some(p) if p == id) {
            let status = if self.caps_active { "On" } else { "Off" };
            let description = if self.caps_active {
                "ALL CAPS typing is active"
            } else {
                "Normal typing mode"
            };

            let content = cosmic::widget::column::with_children(vec![
                text::heading("Caps Lock").into(),
                divider::horizontal::default().into(),
                text::title1(status).into(),
                text::body(description).into(),
            ])
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

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_millis(250))
            .map(|_| Message::CapsLockChanged(keyboard::query_caps_lock()))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cosmic::applet::run::<CapsLockApplet>(())
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}