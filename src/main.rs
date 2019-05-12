#[macro_use]
extern crate log;
extern crate fern;
extern crate serde_json;
extern crate http;

// use nix::sys::signal::*;


use std::env;
use std::thread;
use std::time;
use std::collections::HashMap;

use fern::colors::{Color, ColoredLevelConfig};

mod config;

fn setup_logger() -> Result<(), fern::InitError> {

    let mut colors = ColoredLevelConfig::new()
        // use builder methods
        .info(Color::Green);
        // or access raw fields
        colors.warn = Color::Magenta;

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

enum UOkClientError {
    UrlError(reqwest::UrlError),
    ReqwestError(reqwest::Error),
    MethodError(http::method::InvalidMethod)
}

/* Return status of endpoint
 */
fn check_endpoint(config: &config::UOKEndpointConfig) -> Result<reqwest::Response, UOkClientError> {

    let client = reqwest::Client::new();

    //todo validation of client method


    let method = match config.method {
        // try to parse method string
        Some(ref methodstring) => {
            reqwest::Method::from_bytes(methodstring.as_bytes())
                    .map_err(|err| UOkClientError::MethodError(err))?
        },
        
        // default to GET request if no method supplied
        None => {reqwest::Method::GET}
    };

    

    let url = reqwest::Url::parse(config.url.as_ref())
        .map_err(|err| UOkClientError::UrlError(err))?;

    let request = reqwest::Request::new(method, url);
    return client.execute(request).map_err(|err| UOkClientError::ReqwestError(err));
       
}  

/* Main loop for checking things are ok hun
 */
fn ruokloop(config_file_path: String){

    // initial load of config
    // TODO: more elegant error handling when config is invalid
    let mut conf = config::load_config(&config_file_path);

    let mut last_config_refresh = time::Instant::now();

    let mut last_checked : HashMap<String, time::Instant>  = HashMap::new();

    loop {

        if time::Instant::now() > (last_config_refresh + time::Duration::from_secs(30)) {
            info!("Refreshing config file in {}", &config_file_path);
            conf = config::load_config(&config_file_path);
            last_config_refresh = time::Instant::now()
        }
        

        for endpoint in &conf.endpoints {

            let mut needs_check = false;

            match last_checked.get(&endpoint.url){
                Some(timestamp) => {

                    if time::Instant::now() > ( *timestamp + time::Duration::from_secs(10)){
                        needs_check = true;
                    }

                },
                None => {
                    needs_check = true;
                }
            }

            if needs_check{
                info!("Checking endpoint {}", &(endpoint.url));

                match check_endpoint(endpoint) {
                    Ok(mut result) => {
                        info!("Endpoint {} was responsive", &(endpoint.url));
                        info!("{}", result.text().unwrap());
                    },

                    Err(err) => {
                        match err {
                            UOkClientError::ReqwestError(err) =>{
                                warn!("Oh no hun, {} is not ok! Reason: {}", &(endpoint.url), err)
                            },

                            _ => {
                                error!("Endpoint config for {} is borked.", &(endpoint.url))
                            }
                        }
                    }
                }

                last_checked.insert(endpoint.url.clone(), time::Instant::now());
            }
            


        }

        // at the end of the loop sleep for 1 second
        thread::sleep(time::Duration::from_millis(1000));
    }



}

// extern fn handle_sigint(_:i32) {
//   println!("Interrupted!");
//   panic!();
// }


fn main() {

    
    // let handler = SigHandler::Handler(handle_sigint);
    // let sig_action = SigAction::new(handler,SaFlags::empty(),SigSet::empty());
    // unsafe{
    //     signal::sigaction(signal::SIGINT, &sig_action);
    // }
    
    
    setup_logger().expect("Failed to initialise logger");

    info!("Starting U OK HUN?...");

    let mut config_file = String::new();

    for (key, value) in env::vars() {

        match key.as_ref() {
            "UOKHUN_CONFIG_FILE" => {config_file.clone_from(&value); break;}
            _ => {}
        }
    }

    if config_file.is_empty() {
        panic!("No config file supplied! Did you set UOKHUN_CONFIG_FILE? env variable?")
    }    

    ruokloop(config_file);
}
