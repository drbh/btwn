use std::cell::RefCell;
use std::rc::Rc;

pub trait Middleware<S> {
    fn handle(&self, req: &mut Request, shared_storage: &mut SharedStorage<S>) -> bool;
}

pub struct Request {
    pub path: String,
}

pub struct SharedStorage<S> {
    data: Rc<RefCell<S>>,
}

impl<S> SharedStorage<S> {
    pub fn new(data: S) -> SharedStorage<S> {
        SharedStorage {
            data: Rc::new(RefCell::new(data)),
        }
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<'_, S> {
        self.data.borrow_mut()
    }

    pub fn borrow(&self) -> std::cell::Ref<'_, S> {
        self.data.borrow()
    }
}

pub struct MiddlewareHandler<S> {
    middlewares: Vec<Box<dyn Middleware<S>>>,
    pub shared_storage: SharedStorage<S>,
}

impl<S> MiddlewareHandler<S> {
    pub fn new(data: S) -> MiddlewareHandler<S> {
        MiddlewareHandler {
            middlewares: Vec::new(),
            shared_storage: SharedStorage::new(data),
        }
    }

    pub fn add_middleware(&mut self, middleware: Box<dyn Middleware<S>>) {
        self.middlewares.push(middleware);
    }

    pub fn handle_request(&mut self, mut req: Request) {
        for middleware in &self.middlewares {
            if !middleware.handle(&mut req, &mut self.shared_storage) {
                break;
            }
        }
    }
}
