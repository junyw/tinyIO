use http::{Request, Response, Method};
use std::io::{self};
use std::option;
use regex::{Regex, RegexSet};
use view::{View, NotFound, StaticPage};
#[test]
fn it_works() {
	let mut routerBuilder = RouterBuilder::new();
	let mut router = routerBuilder.get("b", Box::new(NotFound))
								  .get("c", Box::new(NotFound))
								  .post("D", Box::new(NotFound)).build();
	println!("{:?}", router.views[0].render());
	println!("{:?}", router.route(Method::POST, "D"));
}
pub struct RouterBuilder {
	regexs: Vec<&'static str>,
	methods: Vec<Method>,
	views: Vec<Box<View>>,
}
impl RouterBuilder {
	pub fn new() -> RouterBuilder {
		RouterBuilder {
			regexs: Vec::new(),
			methods: Vec::new(),
			views: Vec::new(),
		}
	}
	fn rule(self, method: Method, uri: &'static str, view: Box<View>) -> RouterBuilder {
		match self {
			RouterBuilder {regexs: mut r, methods: mut m, views: mut v} => {
				r.push(uri);
				m.push(method);
				v.push(view);

				RouterBuilder {
					regexs: r,
					methods: m,
					views: v,
				}
			}
		}
	}
	pub fn get(self, uri: &'static str, view: Box<View>) -> RouterBuilder {
		self.rule(Method::GET, uri, view)
	}
	pub fn post(self, uri: &'static str, view: Box<View>) -> RouterBuilder {
		self.rule(Method::POST, uri, view)
	}
	pub fn build(self) -> Router {
		// TODO: check duplicate/confilict routes
		match self {
			RouterBuilder {regexs: mut r, methods: mut m, views: mut v} => {
				Router {
					regexs: RegexSet::new(r).unwrap(),
					methods: m,
					views: v,
				}
			}
		}
	}

}

pub struct Router  {
	//base: String,
	// routes: HashMap<String, Route<View>>,
	regexs: RegexSet,
	methods: Vec<Method>,
	views: Vec<Box<View>>,
}
impl Router {
	pub fn new() -> Router {
		Router {
			regexs: RegexSet::new(&[r"a",r"b"]).unwrap(),
			methods: vec![Method::GET, Method::GET],
			views: vec![Box::new(NotFound), Box::new(NotFound)],
		}
	}
	pub fn response(&self, method: Method, path: &str) -> Response {
		match self.route(method, path) {
			Some(i) => {
				self.views[i].render()
			}
			None => Response::not_found()
		}
	}
	fn route(&self, method: Method, path: &str) -> Option<usize> {
		let matches: Vec<_> = self.regexs.matches(path).into_iter().collect();
		for index in matches {
			if self.methods[index] == method {
				return Some(index);
			}
		}
		return None;
	}
}



