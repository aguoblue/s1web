use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;
use super::handler::{StaticPageHandler, PageNotFoundHandler, WebServiceHandler, Handler};

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> (){
        //打印 route
        println!("route");
        println!("{:?}", req);
        match req.method {
            httprequest::Method::GET => match &req.resource{
                httprequest::Resourse::Path(s) => {
                    //打印下s
                    println!("{}", s);
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            //打印下api
                            println!("api");
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        },
                        _ => {
                            //打印下static
                            println!("static"); 
                            let resp = StaticPageHandler::handle(&req);
                            //打印下resp
                            println!("{:?}", resp);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let resp = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}