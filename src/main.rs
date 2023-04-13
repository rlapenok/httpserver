use httpserver::{req_res_service::ReqResService, service::Service};
use tokio::net::TcpListener;

mod httpserver;

#[tokio::main]
async fn main() {
    
    let listner=TcpListener::bind("127.0.0.1:8080").await.unwrap();

    while let Ok((s,_c)) =listner.accept().await  {

            ReqResService.call(s).await;
    }
}
