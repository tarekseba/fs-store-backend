use crate::config::Config;

pub fn server_running(config: &Config) {
    println!("");
    println!("//////////////////////////////////////");
    println!(
        "// Server running on {}:{} //",
        config.get_srv_addr(),
        config.get_srv_port()
    );
    println!("//////////////////////////////////////");
}
