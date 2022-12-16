use std::collections::HashMap;

struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

struct BasicRouter {
    routes: HashMap<String, BoxedCallback>
}

impl BasicRouter {
    // Create an empty router.
    fn new() -> BasicRouter {
        BasicRouter { routes: HashMap::new() }
    }

    // Add a route to the router.
    fn add_route<C>(&mut self, url: &str, callback: C)
        where C: Fn(&Request) -> Response + 'static
    {
        self.routes.insert(url.to_string(), Box::new(callback));
    }
}

impl BasicRouter {
    fn handle_request(&self, request: &Request) -> Response {
        match self.routes.get(&request.url) {
            None => not_found_response(),
            Some(callback) => callback(request)
        }
    }
}

#[test]
fn test_router() {
    let mut router = BasicRouter::new();
    router.add_route("/", |_| get_form_response());
    router.add_route("/gcd", |req| get_gcd_response(req));
    let response = router.handle_request(&Request {
        method: "post".to_string(),
        url: "/gcd".to_string(),
        headers: Default::default(),
        body: vec![]
    });
    assert_eq!(response.code, 13);
}

fn get_form_response() -> Response {
    Response {
        code: 0,
        headers: Default::default(),
        body: vec![]
    }
}

fn not_found_response() -> Response {
    get_form_response()
}

fn get_gcd_response(_request: &Request) -> Response {
    Response {
        code: 13,
        headers: Default::default(),
        body: vec![]
    }
}