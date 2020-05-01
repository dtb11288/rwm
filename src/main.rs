use rwm::config::Config;
use rwm::manager::Manager;
use simple_logger;

fn main() {
    simple_logger::init().unwrap();
    Manager::new(Config::new()).run()
}
