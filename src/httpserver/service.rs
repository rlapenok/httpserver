use std::future::Future;



pub trait Service<Request>

{
    type Future:Future;
    fn call(&mut self,request:Request)->Self::Future;
}