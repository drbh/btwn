# btwn (Between)

btwn is a middleware-focused Rust library that provides users with a flexible and intuitive interface for managing and handling requests in Rust applications. It offers a unique trait-based approach to creating and applying middleware to requests, as well as an easy-to-use SharedStorage system for managing shared data.

### Installation

You can add btwn to your project by adding the following lines to your Cargo.toml:

```toml
[dependencies]
btwn = "0.1.0"
```

### Core Concepts

#### Middleware

`btwn` is centered around the concept of middleware - reusable units of code that can be applied to requests to handle tasks such as logging, authentication, and much more. In `btwn`, middleware are implemented as traits, which makes them highly flexible and easy to implement.

#### SharedStorage

`btwn` also provides a `SharedStorage` struct, which is a thread-safe storage system that allows data to be shared and mutated between different middleware.

### Usage

To use `btwn`, you'll need to create a struct that implements the `Middleware` trait, which consists of a single function, `handle`. This function takes a mutable reference to a `Request` object and a mutable reference to a `SharedStorage` object and returns a boolean. If `handle` returns false, the processing of the current request is stopped.

```rust
struct MyMiddleware;

impl Middleware<()> for MyMiddleware {
    fn handle(&self, req: &mut Request, shared_storage: &mut SharedStorage<()>) -> bool {
        // Middleware logic goes here...
        true
    }
}
```

Once you've implemented your middleware, you can add it to a `MiddlewareHandler` object. This object handles the application of middleware to requests.

```rust
let mut handler = MiddlewareHandler::new(());
handler.add_middleware(Box::new(MyMiddleware));
```

To handle a request, simply call the handle_request function on your `MiddlewareHandler` object, passing it a `Request` object:

```rust
let request = Request { path: String::from("/my/path") };
handler.handle_request(request);
```

This will apply all added middleware to the request, in the order they were added.

### Contributing
We welcome contributions to btwn! Please feel free to open an issue or submit a pull request if you have any ideas, suggestions, or improvements.

### License
`btwn` is licensed under the MIT license. Please see the LICENSE file for more details.
