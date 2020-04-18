use rwm::display::xcb_server::XcbDisplay;
use rwm::config::Config;
use rwm::manager::Manager;

fn main() {
    let manager: Manager<XcbDisplay> = Manager::new(Config {});
    manager.run()
}
