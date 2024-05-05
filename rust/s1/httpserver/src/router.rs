use super::handler::{Handler, PageNotFoundHandler, StaticOageHandler, WebServiceHandler};
use http::{
    httprequest,httprequest::HttpRequest,httpresponse::HttpResponse
};
use std::io::prelude::*;


pub struct Router;

impl Router {
    pub fn route(req:HttpRequest,stream: &mut impl Write)->(){
        match req.method {
            httprequest::Method::Get => match &req.resource  {
                httprequest::Resurce::Path(s) => {
                    let route:Vec<&str> = s.split("/").collect();
                    match route[1] {
                       "api" => {
                        let resp :HttpResponse = WebServiceHandler::handle(&req);
                        let _ = resp.send_response(stream);
                       },
                       _ =>{
                        let resp :HttpResponse = StaticOageHandler::handle(&req);
                        let _ = resp.send_response(stream);
                       }
                    }
                }
            },
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}