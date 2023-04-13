use std::{future::Future, pin::Pin};

use saf_httparser::request_from_bytes;
use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};

use super::{service::Service, open_file_service::OpenFileService};

pub struct ReqResService;


impl Service<TcpStream> for ReqResService

{
    type Future = Pin<Box<dyn Future<Output = ()>>>;
    fn call(&mut self,mut request:TcpStream)->Self::Future {
        
        let c=async move {

            let mut buff=[0;1024];
            request.read(&mut buff).await.unwrap();
            
           let open_file= match request_from_bytes(&buff).unwrap().uri().path(){
                "/"=>{
                    OpenFileService.call("hello.html").await
                }
                "/login-page.css"=>{
                    OpenFileService.call("login-page.css").await
                }
                _=>{OpenFileService.call("404.html").await}

            };
            println!("{:?}",request_from_bytes(&buff).unwrap());
            let resp=format!("HTTP/1.1 200 OK\r\n\r\n{}",open_file);
            request.write(resp.as_bytes()).await.unwrap();
            request.flush().await.unwrap();

        } ;
        Box::pin(c)
    }
}
