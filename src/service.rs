
use hyper::Get;
use hyper::header::{ContentType;
use hyper::server::{Request, Response, Handler};
use hyper::uri::RequestUri::AbsolutePath;

struct Service;

impl Handler for Service {
  fn handle(&self, req: Request, mut res: Response) {
        match req.uri {
            AbsolutePath(ref path) => {
                match (&req.method, &path.split("?").collect::<Vec<_>>()[0][..]) {
                  {{~#eacher paths}}
                  (_, "{{@key}}") => {
                    /// {{lower this}}
                  },
                  {{/eacher~}}
                    (method, path) => {
                      println!("hit with unsupported path {} {}", method, path);
                      res.headers_mut().set(ContentType::json());
                      *res.status_mut() = hyper::NotFound;
                      return;
                    }
                }
            }
            _ => (),
        }
    }
}
