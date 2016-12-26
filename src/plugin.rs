use std::process;

use dokku::Dokku;
use config::Config;

pub struct Plugin {
    dokku: Dokku,
    config: Config,
}

#[derive(Debug)]
pub enum Command {
    Install,
    NotImplemented,
    Empty,
}

impl Plugin {
    pub fn new(dokku: Dokku, config: Config) -> Plugin {
        Plugin {
            dokku: dokku,
            config: config,
        }
    }

    pub fn run(&self, command: Command) {
        println!("Command: {:?}", command);

        match command {
            Command::Install => self.install(),
            Command::Empty => process::exit(self.dokku.env.DOKKU_VALID_EXIT),
            Command::NotImplemented => process::exit(self.dokku.env.DOKKU_NOT_IMPLEMENTED_EXIT),
        }
    }

    pub fn install(&self) {
        println!("INSTALL");
        self.dokku.pull_docker_image(format!("{}:{}", self.config.image, self.config.version))
    }
}
