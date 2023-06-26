use btwn::{Middleware, MiddlewareHandler, Request, SharedStorage};

pub struct Shared {
    all_text: Vec<String>,
    current: Option<Vec<String>>,
}

pub struct FirstMiddleware;

impl Middleware<Shared> for FirstMiddleware {
    fn handle(&self, req: &mut Request, shared_storage: &mut SharedStorage<Shared>) -> bool {
        if req.path == "/test" {
            let mut shared_data = shared_storage.borrow_mut();
            shared_data.all_text.push("This is a test".to_string());
            shared_data.current = Some(vec!["First middleware".to_string()]);
            return true;
        }
        false
    }
}

pub struct SecondMiddleware;

impl Middleware<Shared> for SecondMiddleware {
    fn handle(&self, req: &mut Request, shared_storage: &mut SharedStorage<Shared>) -> bool {
        if req.path == "/test" {
            let mut shared_data = shared_storage.borrow_mut();

            // update the shared data
            shared_data
                .current
                .as_mut()
                .unwrap()
                .push("Second middleware".to_string());


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
    handler.add_middleware(Box::new(FirstMiddleware));
    handler.add_middleware(Box::new(SecondMiddleware));

    handler.handle_request(Request {
        path: "/test".to_string(),
    });

    let shared_data = handler.shared_storage.borrow();
    assert_eq!(shared_data.all_text[0], "This is a test".to_string());
    assert_eq!(
        shared_data.current.as_ref().unwrap()[1],
        "Second middleware".to_string()
    );

    // print out the result to stdout
    println!("{:?}", shared_data.all_text);
    println!("{:?}", shared_data.current);
}
