mod system_interface;
mod explorer;
use system_interface::windows_interface;

fn main() {
    let mut explorer = explorer::Explorer::new(Box::new(windows_interface::WindowsInterface::new()));
    explorer.load_file();
    println!("Mom: {:?}", explorer.get_file());
    explorer.select_file(0);
    println!("Mom: {:?}", explorer.get_file());
    explorer.go_prev();
    println!("Mom: {:?}", explorer.get_file());
}
