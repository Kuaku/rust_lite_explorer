mod system_interface;
mod explorer;
mod display;
use system_interface::windows_interface;

fn main() {
    let mut display = display::Display::new(explorer::Explorer::new(Box::new(windows_interface::WindowsInterface::new())));
    loop {
        display.draw();
        display.update();
    }
}
