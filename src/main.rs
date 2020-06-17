#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;
use std::io;
use std::io::Write;
use std::borrow::Borrow;
mod occwrapper;
use occwrapper::Occ;

const USAGE: &'static str = "
ownCloud enable or disable app

Usage:
  indrajal listapp
  indrajal enableapp <appid>
  indrajal disableapp <appid>
  indrajal (-h | --help)
  indrajal --version

options:
  -h --help   Show this screen
  --version   Show version
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_enableapp: bool,
    cmd_disableapp: bool,
    cmd_listapp: bool,
    arg_appid: String,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    //println!("{:?}", args);

    if args.cmd_enableapp {
        println!("enable app called with id {:?}", args.arg_appid);
        let mut occInstance = Occ {occPath: String::from("")};
        let mut occPathFromCwd = occInstance.getCWD();
        occPathFromCwd.push_str("/../../occ");
        occInstance.setOccPath(occPathFromCwd);

        let occClone = occInstance.clone();
        if occClone.verifyOccExist() == false {
            println!("occ file not found hence aborting");
            return();
        }

        //println!("{:?}", occInstance.enableapp(args.arg_appid.as_str()));
        //let mut enableApp = occInstance.enableapp(args.arg_appid.as_str());
        println!("{}", String::from_utf8_lossy(&occInstance.enableapp(args.arg_appid.as_str()).stdout));
        //println!("{}", enableApp.status);*/
    } else if args.cmd_disableapp {
        println!("disable app called with id {:?}", args.arg_appid);
        let mut occInstance = Occ {occPath: String::from("")};
        let mut occPathFromCwd = occInstance.getCWD();
        occPathFromCwd.push_str("/../../occ");
        occInstance.setOccPath(occPathFromCwd);
        
        let occClone = occInstance.clone();
        if occClone.verifyOccExist() == false {
            println!("occ file not found hence aborting");
            return();
        }

        //println!("{:?}", occInstance.disableapp(args.arg_appid.as_str()));
        println!("{}", String::from_utf8_lossy(&occInstance.disableapp(args.arg_appid.as_str()).stdout));
    } else if args.cmd_listapp {
        let mut occInstance = Occ {occPath: String::from("")};
        let mut occPathFromCwd = occInstance.getCWD();
        occPathFromCwd.push_str("/../../occ");
        occInstance.setOccPath(occPathFromCwd);

        let occClone = occInstance.clone();
        if occClone.verifyOccExist() == false {
            println!("occ file not found hence aborting");
            return();
        }

        println!("{}", String::from_utf8_lossy(&occInstance.listApps().stdout));        
    } else if args.flag_version {
        println!("{}", env!("CARGO_PKG_VERSION"));
        //println!("{:?}", Occ.getPath());
    }
}

