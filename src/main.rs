use std::sync::mpsc;

pub enum Event {
    Rpc(JsonRpc),
}

pub enum Id {
    Str(String),
}

pub fn response<I: Into<Id>>(id: I) -> JsonRpc {
    JsonRpc::Response(Response {
        id: id.into(),
        method: "request".into(),
    })
}

struct Response {
    id: Id,
    method: String,
}

pub enum JsonRpc {
    Response(Response),
}

fn main() {
    let (tx, rx) = mpsc::channel();
    for evt in rx {
        match evt {
            Event::Rpc(JsonRpc::Response(ref resp)) if resp.method == "shutdown" => {}
            Event::Rpc(JsonRpc::Response(ref resp)) if resp.method == "request" => {
                tx.send(Event::Rpc(response(resp.id))).unwrap();
            }
            _ => (),
        };
    }
}
