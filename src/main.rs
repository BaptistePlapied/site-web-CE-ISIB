use topcoat::{
    router::{module_router, page},
    view::view,
    Result,
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
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Mon lecteur YouTube"</title>
            </head>
            <body>
                <h1>"Y a pas de panneau"</h1>


                <iframe
                    width="560"
                    height="315"
                    src="https://www.youtube.com/embed/2gDivktaWmY"
                    title="Lecteur YouTube"
                    allow="autoplay; encrypted-media; picture-in-picture"
                    referrerpolicy="strict-origin-when-cross-origin"
                    allowfullscreen="true"
                ></iframe>

                <p>
                    <a href="https://www.youtube.com/watch?v=2gDivktaWmY">
                        "Ouvrir directement sur YouTube"
                    </a>
                </p>
            </body>
        </html>
    }
}
