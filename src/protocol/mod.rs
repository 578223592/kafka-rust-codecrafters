pub mod v0; // Protocol Version 0 ， use this file to let the compiler know that there is a file named v0.rs
pub mod v2; // Protocol Version 2



// 不同response的trait
pub(crate) trait ResponseWorker {
    fn new(requset_parser: v2::RequsetParserV2) -> Self where Self: Sized; //todo 待把requset_parser抽象成trait
    fn build_response(&mut self) -> Vec<u8>;
}



pub fn response_worker_factory(requset_parser: v2::RequsetParserV2) -> Box<dyn ResponseWorker> {
    match requset_parser.get_request_api_key() {
        18 => Box::new(v0::ApiVersionsWorker::new(requset_parser)),
        _ => panic!("Unsupported request api key"),
    } 
}