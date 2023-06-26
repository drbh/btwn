use btwn::{Request, Middleware, SharedStorage, MiddlewareHandler};

pub struct Shared {
    all_text: Vec<String>,
    current: Option<Vec<String>>,
}

pub struct TestMiddleware;

impl Middleware<Shared> for TestMiddleware {
    fn handle(&self, req: &mut Request, shared_storage: &mut SharedStorage<Shared>) -> bool {
        if req.path == "/test" {
            let mut shared_data = shared_storage.borrow_mut();
            shared_data.all_text.push("This is a test".to_string());
            shared_data.current = Some(vec!["Current text".to_string()]);
            return true;
        }
        false
    }
}

fn main() {
    let mut handler = MiddlewareHandler::new(Shared {
        all_text: Vec::new(),
        current: None,
    });
    handler.add_middleware(Box::new(TestMiddleware));

    handler.handle_request(Request { path: "/test".to_string() });

    let shared_data = handler.shared_storage.borrow();
    assert_eq!(shared_data.all_text[0], "This is a test".to_string());
    assert_eq!(shared_data.current.as_ref().unwrap()[0], "Current text".to_string());

    // print out the result to stdout
    println!("{:?}", shared_data.all_text);
    println!("{:?}", shared_data.current);
}
