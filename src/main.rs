use procii::gui::ApplicationState;

fn main() -> iced::Result {
    iced::run("Procii", ApplicationState::update, ApplicationState::view)
}
