use log::{error, info};

pub fn ha() {
    match std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo systemctl start docker && cd $HOME/docker  && docker compose up -d")
        .output()
    {
        Ok(_) => info!("on_init went correctly"),
        Err(out) => error!("on_init failed due to: {out}"),
    }
}
