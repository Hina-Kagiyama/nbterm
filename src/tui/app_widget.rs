use ratatui::widgets::StatefulWidget;

#[derive(Default)]
pub struct App {}

#[derive(Default)]
pub struct AppState {}

impl StatefulWidget for &App {
    type State = AppState;

    fn render(
        self,
        _area: ratatui::prelude::Rect,
        _buf: &mut ratatui::prelude::Buffer,
        _state: &mut Self::State,
    ) {
    }
}
