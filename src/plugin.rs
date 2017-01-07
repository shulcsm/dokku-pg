use std::process;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

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

pub struct Service<'a, 'b> {
    plugin: &'a Plugin,
    name: &'b str,
    root: PathBuf,
}

impl<'a, 'b> Service<'a, 'b> {
    pub fn new(plugin: &'a Plugin, name: &'b str) -> Service<'a, 'b> {
        let root = Path::new(&plugin.config.root).join(name);

        // image??
        Service {
            plugin: plugin,
            name: name,
            root: root,
        }
    }

    pub fn exists(&self) -> bool {
        self.root.exists()
    }

    pub fn create(&self, image: &str) -> PluginResult {
        // ensure image
        self.plugin.dokku.pull_docker_image(image);

        fs::create_dir_all(&self.root)?;
        fs::create_dir_all(&self.root.join("data"))?;
        fs::create_dir_all(&self.root.join("config"))?;

        // postgres password
        // custom env, links
        // create container
        // echo "dokku.${PLUGIN_COMMAND_PREFIX}.$SERVICE"

        Ok(())
    }

    pub fn destroy(&self) {
        //
    }
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

    pub fn create(&self, name: &str, image: &str, port: Option<&str>) -> PluginResult {
        let service = Service::new(self, name);

        println!("Create service: {:?}, {:?}, {:?}", name, image, port);
        if service.exists() {
            return Err(PluginError::UnexpectedState {
                msg: format!("PG service {} already exists.", name),
            });
        }

        service.create(image)
    }

    pub fn no_command(&self) -> PluginResult {
        Ok(())
    }

    pub fn not_implemented(&self) -> PluginResult {
        Err(PluginError::NotImplemented)
    }

    pub fn install(&self) -> PluginResult {
        println!("INSTALL");
        self.dokku.pull_docker_image(&self.default_image);
        self.dokku.pull_docker_image("dokkupaas/wait:0.2");

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
