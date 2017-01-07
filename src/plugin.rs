use std::process;
use std::fs;
use std::io;
use std::path::Path;

use dokku::Dokku;
use config::Config;

use util;

pub struct Plugin {
    dokku: Dokku,
    config: Config,
    pub default_image: String,
}

#[derive(Debug)]
pub enum PluginError {
    Io(io::Error),
    NotImplemented,
    UnexpectedState { msg: String },
}

impl From<io::Error> for PluginError {
    fn from(err: io::Error) -> PluginError {
        PluginError::Io(err)
    }
}


type PluginResult = Result<(), PluginError>;

impl Plugin {
    pub fn new(dokku: Dokku, config: Config) -> Plugin {
        let default_image = format!("{}:{}", config.image, config.version);
        Plugin {
            dokku: dokku,
            config: config,
            default_image: default_image,
        }
    }

    pub fn create(&self, name: &str, image: &str, port: Option<&str>) -> PluginResult {
        println!("Create service: {:?}, {:?}, {:?}", name, image, port);
        let path = Path::new(&self.config.root).join(name);

        if path.exists() {
            return Err(PluginError::UnexpectedState {
                msg: format!("PG service {} already exists.", name),
            });
        }
        Ok(())
    }

    pub fn no_command(&self) -> PluginResult {
        Ok(())
    }

    pub fn not_implemented(&self) -> PluginResult {
        Err(PluginError::NotImplemented)
    }

    pub fn install(&self) -> PluginResult {
        println!("INSTALL");
        self.dokku.pull_docker_image(format!("{}:{}", self.config.image, self.config.version));
        self.dokku.pull_docker_image("dokkupaas/wait:0.2".to_string());
        let root = Path::new(&self.config.root);
        fs::create_dir_all(&root)?;
        util::chown_by_name(&root, "dokku", "dokku")?;

        Ok(())
    }

    pub fn exit(&self, result: PluginResult) -> ! {
        match result {
            Ok(..) => process::exit(self.dokku.env.DOKKU_VALID_EXIT),
            Err(PluginError::NotImplemented) => {
                process::exit(self.dokku.env.DOKKU_NOT_IMPLEMENTED_EXIT)
            }
            Err(e) => {
                println!("ERROR: {:?}", e);
                process::exit(1);
            }
        }
    }
}
