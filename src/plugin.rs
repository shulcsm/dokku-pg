use std::process;
use std::fs;
use std::io::{self, Write, Read, BufReader, BufRead};
use std::path::{Path, PathBuf};
use shiplift::ContainerOptions;

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

pub struct Service<'a> {
    plugin: &'a Plugin,
    name: String,
    image: String,
    root_dir: PathBuf,
    data_dir: PathBuf,
    config_dir: PathBuf
}

impl<'a> Service<'a> {
    pub fn new(plugin: &'a Plugin, name: &str, image: &str) -> Service<'a> {
        let root = Path::new(&plugin.config.root).join(name);
        let data = root.join("data");
        let config = root.join("config");

        // image??
        Service {
            plugin: plugin,
            name: name.to_owned(),
            image: image.to_owned(),
            root_dir: root,
            data_dir: data,
            config_dir: config,
        }
    }

    pub fn installed(&self) -> bool {
        self.root_dir.exists()
    }

    pub fn get_password(&self) -> Result<String, PluginError> {
        let mut password = String::new();
        fs::File::open(&self.root_dir.join("PASSWORD"))?.read_to_string(&mut password);
        Ok(password)
    }

    fn build_env(&self) -> Result<Vec<String>, PluginError> {
        let mut env = Vec::new();
        env.push(format!("POSTGRES_PASSWORD={}", self.get_password()?));
        let env_file = fs::File::open(&self.root_dir.join("PASSWORD"))?;

        for line in BufReader::new(&env_file).lines() {
            env.push(line?);
        }
        Ok(env)
    }

    pub fn container_options(&self) -> Result<ContainerOptions, PluginError> {
        let data_volume = format!("{}:{}", self.data_dir.to_str().unwrap(), "/var/lib/postgresql/data");
        let env = self.build_env()?;

        // @TODO labels
        Ok(ContainerOptions::builder(&self.image)
           .name(&self.name)
           .volumes(vec![&data_volume])
           .env(self.build_env()?.iter().map(|s| &**s).collect())
           .build()
        )
    }

    pub fn install(&self) -> PluginResult {

        fs::create_dir_all(&self.root_dir)?;
        fs::create_dir_all(&self.data_dir)?;
        fs::create_dir_all(&self.config_dir)?;

        let pass = util::generate_password();
        fs::File::create(&self.root_dir.join("PASSWORD"))?.write_all(pass.as_bytes())?;
        // @TODO
        fs::File::create(&self.root_dir.join("ENV"))?;

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
        let service = Service::new(self, name, image);

        println!("Create service: {:?}, {:?}, {:?}", name, image, port);

        if service.installed() {
            Err(PluginError::UnexpectedState {
                msg: format!("PG service {} already exists.", name),
            })
        } else {
            service.install()?;
            // ensure image
            self.dokku.pull_docker_image(image);
            let opts = service.container_options()?;
            let res = self.dokku.docker.containers().create(&opts);
            println!("{:?}", res);
            Ok(())
        }
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
