use rwm::config::Config;
use rwm::manager::Manager;
use simple_logger;
use rwm::displays::xcb_server::XcbDisplayServer;

#[tokio::main]
async fn main() {
    simple_logger::init().unwrap();
    Manager::<XcbDisplayServer>::new(Config::new()).stream().await;
}
