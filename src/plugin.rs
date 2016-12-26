use std::process;
use std::fs;
use std::path::Path;

use dokku::Dokku;
use config::Config;


pub struct Plugin {
    dokku: Dokku,
    config: Config,
    pub default_image: String,
}

impl Plugin {
    pub fn new(dokku: Dokku, config: Config) -> Plugin {
        let default_image = format!("{}:{}", config.image, config.version);
        Plugin {
            dokku: dokku,
            config: config,
            default_image: default_image,
        }
    }

    pub fn create(&self, name: &str, image: &str, port: Option<&str>) {
        println!("Create service: {:?}, {:?}, {:?}", name, image, port);
        let path = Path::new(&self.config.root).join(name);

        if path.exists() {
            self.dokku.log_fail(format!("PG service {} already exists.", name));
        }
    }

    pub fn no_command(&self) -> ! {
        process::exit(self.dokku.env.DOKKU_VALID_EXIT)
    }

    pub fn not_implemented(&self) -> ! {
        process::exit(self.dokku.env.DOKKU_NOT_IMPLEMENTED_EXIT)
    }

    pub fn install(&self) {
        println!("INSTALL");
        self.dokku.pull_docker_image(format!("{}:{}", self.config.image, self.config.version))
    }
}
