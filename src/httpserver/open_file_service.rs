use std::{pin::Pin, future::Future};

use tokio::fs;

use super::service::Service;


pub struct OpenFileService;

impl Service<&'static str> for OpenFileService{

    type Future = Pin<Box<dyn Future<Output = String>>>;

    fn call(&mut self,request:&'static str)->Self::Future {
        
        let future=async move{
                fs::read_to_string(request).await.unwrap()
        };

        Box::pin(future)
    }

}