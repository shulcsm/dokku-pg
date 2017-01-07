extern crate envy;

use self::envy::Error;

use shiplift::errors::Error as DockerError;
use hyper::status::StatusCode;
use shiplift::{Docker, PullOptions};


// https://github.com/dokku/dokku/blob/7e25e747792bcb8d9d188d956823a1ac2460aab2/dokku
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct DokkuEnv {
    pub DOKKU_ROOT: String,

    #[serde(default = "default_flag")]
    pub DOKKU_TRACE: bool,

    #[serde(default = "default_flag")]
    pub DOKKU_QUIET_OUTPUT: bool,
    pub DOKKU_DISTRO: String,
    pub DOKKU_IMAGE: String,
    pub DOKKU_LIB_ROOT: String,

    pub PLUGIN_PATH: String,
    pub PLUGIN_AVAILABLE_PATH: String,
    pub PLUGIN_ENABLED_PATH: String,
    pub PLUGIN_CORE_PATH: String,

    pub PLUGIN_CORE_AVAILABLE_PATH: String,
    pub PLUGIN_CORE_ENABLED_PATH: String,

    pub DOKKU_API_VERSION: i32,
    pub DOKKU_NOT_IMPLEMENTED_EXIT: i32,
    pub DOKKU_VALID_EXIT: i32,

    pub DOKKU_LOGS_DIR: String,
    pub DOKKU_EVENTS_LOGFILE: String,

    pub DOKKU_CONTAINER_LABEL: String,
    pub DOKKU_GLOBAL_RUN_ARGS: String,
}

fn default_flag() -> bool {
    false
}

pub struct Dokku {
    pub env: DokkuEnv,
    pub docker: Docker,
}

impl Dokku {
    pub fn new() -> Result<Dokku, Error> {
        let env = envy::from_env::<DokkuEnv>();
        Ok(Dokku {
            env: env.unwrap(),
            docker: Docker::new(),
        })
    }

    pub fn image_exists(&self, image: &str) -> bool {
        match self.docker
            .images()
            .get(image)
            .inspect() {
            Ok(..) => true,
            Err(DockerError::Fault { code: StatusCode::NotFound, .. }) => false,
            Err(e) => panic!("{:?}", e),
        }
    }

    pub fn pull_docker_image(&self, image: &str) {
        println!("Fetching image: {}", image);

        if self.image_exists(&image) {
            println!("Image exists.")
        } else {
            println!("Pulling image.");
            let _image = self.docker
                .images()
                .pull(&PullOptions::builder().image(image).build())
                .unwrap()
                .collect::<Vec<_>>();
        }

    }
}
