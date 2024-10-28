#![allow(non_upper_case_globals)]
use futures_util::StreamExt;
use hypertext::{html_elements, maud, Attribute, GlobalAttributes, Renderable};
use poem::{
    endpoint::StaticFilesEndpoint,
    get, handler,
    listener::TcpListener,
    post,
    session::{CookieConfig, CookieSession, Session},
    web::{
        sse::{Event, SSE},
        Html,
    },
    EndpointExt, Route, Server,
};
use std::time::Instant;
use tokio::time::{interval, Duration};
use tokio_stream::wrappers::IntervalStream;

#[allow(dead_code)]
trait HtmxAttributes: GlobalAttributes {
    const hx_post: Attribute = Attribute;
    const hx_target: Attribute = Attribute;
    const hx_swap: Attribute = Attribute;
    const hx_ext: Attribute = Attribute;
    const sse_connect: Attribute = Attribute;
    const sse_swap: Attribute = Attribute;
}

impl<T: GlobalAttributes> HtmxAttributes for T {}

#[tokio::main]
async fn main() {
    let app = Route::new()
        .at("/", get(hello_world))
        .at("/timer", get(timer))
        .at("/increment", post(increment))
        .at("/decrement", post(decrement))
        .nest("/public", StaticFilesEndpoint::new("./public"))
        .with(CookieSession::new(CookieConfig::default().secure(false)));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
        .unwrap()
}

#[handler]
async fn hello_world(session: &Session) -> Html<String> {
    let count = session.get::<i32>("axum.count").unwrap_or(0);

    let template = maud! {
        !DOCTYPE
        html {
            head {
                title { "Hello, World!" }
                link rel="stylesheet" href="/public/style.css" {}
                script src="https://unpkg.com/htmx.org@2.0.3" integrity="sha384-0895/pl2MU10Hqc6jd4RvrthNlDiE9U1tWmX7WRESftEDRosgxNsQG/Ze9YMRzHq" crossorigin="anonymous" {}
                script src="https://unpkg.com/htmx-ext-sse@2.2.2/sse.js" {}
            }
            body class="flex flex-col gap-2 p-4" hx-ext="sse" sse-connect="/timer" {
                h1 { "Hello, World!" }
                p { "Welcome to my website!" }
                p {
                    span sse-swap="timer" {
                        "0"
                    }
                    " seconds since the page was loaded."
                }
                div class="flex gap-2" {
                    button class="border rounded-md px-4 py-2 hover:bg-slate-100" hx-post="/decrement" hx-target="#counter" { "-" }
                    div id="counter" class="flex justify-center bg-slate-100 rounded-md px-4 py-2 w-16" { (count) }
                    button class="border rounded-md px-4 py-2 hover:bg-slate-100" hx-post="/increment" hx-target="#counter" { "+" }
                }
            }
        }
    };

    Html(template.render().0)
}

#[handler]
async fn increment(session: &Session) -> String {
    let count = session.get::<i32>("axum.count").unwrap_or(0) + 1;
    session.set("axum.count", count);

    count.to_string()
}

#[handler]
async fn decrement(session: &Session) -> String {
    let count = session.get::<i32>("axum.count").unwrap_or(0) - 1;
    session.set("axum.count", count);

    count.to_string()
}

#[handler]
fn timer() -> SSE {
    let now = Instant::now();

    let timer_stream = IntervalStream::new(interval(Duration::from_secs(1)));

    let event_stream = timer_stream.map(move |_| Event::Message {
        id: String::new(),
        event: String::from("timer"),
        data: now.elapsed().as_secs().to_string(),
    });

    SSE::new(event_stream).keep_alive(Duration::from_secs(365 * 24 * 60 * 60))
}
