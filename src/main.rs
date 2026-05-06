use crossterm::event;

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| {
                frame.render_widget("Hello!", frame.area());
            })?;
            if event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })
}
