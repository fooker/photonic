use iron::{error::HttpResult, Iron, IronResult, Listening, Request, Response, status::Status};
use photonic::attributes::{Attribute, DynamicValue};
use photonic::attributes::dynamic::{ButtonValue, FaderValue};
use photonic::core::Node;
use photonic::inspection;
use router::Router;
use std::collections::HashMap;
use std::io::Read;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;


pub struct ExporterConfig<'c> {
    pub address: &'c str,
}

pub fn serve(config: ExporterConfig, root_node: &Node) -> HttpResult<Listening> {
    let mut router = Router::new();

    inspection::recurse_attributes(root_node, &mut |attr| {
        if let Attribute::Dynamic { ref name, ref value } = attr {
            {
                let value = value.clone();
                router.get(format!("/value/{}", name), move |req: &mut Request| -> IronResult<Response> {
                    let v = value.lock().unwrap().value();
                    return Ok(Response::with((Status::Ok, v.to_string())));
                }, format!("value-{}-get", name));
            }

            {
                let value = value.clone();
                router.post(format!("/value/{}", name), move |req: &mut Request| -> IronResult<Response> {
                    match &mut *value.lock().unwrap() {
                        DynamicValue::Fader(ref mut dynamic) => {
                            let mut v = String::new();
                            req.body.by_ref().take(64).read_to_string(&mut v);

                            match v.parse::<f64>() {
                                Ok(v) => {
                                    dynamic.set(v);
                                    return Ok(Response::with(Status::Ok));
                                }
                                Err(err) => {
                                    return Ok(Response::with((Status::BadRequest, err.to_string())));
                                }
                            }
                        }
                        DynamicValue::Button(ref mut dynamic) => {
                            dynamic.trigger();
                            return Ok(Response::with(Status::Ok));
                        }
                    }
                }, format!("value-{}-post", name));
            }
        }
    });

    return Iron::new(router).http(config.address);
}

