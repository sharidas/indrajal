use std::env;
use std::path::Path;
use std::process::Command;
use std::process::Output;

#[derive(Clone)]
pub struct Occ {
    pub occPath:  String,
}

impl Occ {
    pub fn getCWD(&self) -> String {
        let p = env::current_dir().unwrap();
        p.into_os_string().into_string().unwrap().to_string()
    }

    pub fn setOccPath(&mut self, path: String)  {
        self.occPath = path;
    }

    pub fn verifyOccExist(self) -> bool {
        Path::new(&self.occPath).exists()
    }

    pub fn listApps(self) -> Output {
        Command::new(self.occPath)
            .arg("app:list")
            .output()
            .expect("failed to execute")
    }

    pub fn enableapp(self, appId: &str) -> Output {
        Command::new(self.occPath)
            .arg("app:enable")
            .arg(appId)
            .output()
            .expect("failed to enable app")
    }

    pub fn disableapp(self, appId: &str) -> Output {
        Command::new(self.occPath)
            .arg("app:disable")
            .arg(appId)
            .output()
            .expect("failed to enable app")
    }    
}

//impl Copy for Occ {}

