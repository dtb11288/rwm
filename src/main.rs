use rwm::config::Config;
use rwm::manager::Manager;
use rwm::displays::xcb_server::XcbDisplayServer;
use simple_logger;

fn main() {
    simple_logger::init().unwrap();
    let manager: Manager<XcbDisplayServer> = Manager::new(Config::new());
    manager.run()
}
