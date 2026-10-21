use topcoat::{
    Result,
    router::{module_router, page},
    view::{component, view},
};

#[tokio::main]
async fn main() {
    topcoat::start(module_router!().build()).await.unwrap();
}

#[page]
async fn home() -> Result {
    view! {
        <!DOCTYPE html>
        <html>
            <body>
                hello(name: "World")
            </body>
        </html>
    }
}

#[component]
async fn hello(name: &str) -> Result {
    view! { <h1>"Hello, " (name) "!"</h1> }
}
