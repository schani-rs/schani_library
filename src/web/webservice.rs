use fern;
use gotham::handler::NewHandlerService;
use hyper::server::Http;

use std::io;

use super::router::build_app_router;

use log::LogLevelFilter;

pub struct LibraryWebService<'a> {
    database_url: &'a str,
}

impl<'a> LibraryWebService<'a> {
    pub fn new(database_url: &'a str) -> Self {
        LibraryWebService {
            database_url: database_url,
        }
    }

    fn set_logging(&self) {
        fern::Dispatch::new()
            .level(LogLevelFilter::Info)
            .chain(io::stdout())
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{}][{}]{}",
                    record.target(),
                    record.level(),
                    message
                ))
            })
            .apply()
            .unwrap();
    }

    pub fn run(self) {
        self.set_logging();

        let addr = "0.0.0.0:8002".parse().unwrap();
        trace!("create router");
        let router = build_app_router(self.database_url);
        trace!("create server");
        let server = Http::new()
            .bind(&addr, NewHandlerService::new(router))
            .unwrap();

        info!("server listening on {}", addr);
        server.run().unwrap();
    }
}
