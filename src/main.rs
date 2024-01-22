use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use strum::EnumString;
use tower_http::services::ServeDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum ReactNode {
    ReactElement(ReactElement),
    Multiple(Vec<ReactNode>),
    String(String),
    Number(f64),
    Boolean(bool),
}

//TODO: implement more types
impl From<&str> for Box<ReactNode> {
    fn from(value: &str) -> Self {
        Box::new(ReactNode::String(value.to_string()))
    }
}

impl From<&str> for ReactNode {
    fn from(value: &str) -> Self {
        ReactNode::String(value.to_string())
    }
}

impl From<String> for ReactNode {
    fn from(value: String) -> Self {
        ReactNode::String(value)
    }
}

impl From<f64> for ReactNode {
    fn from(value: f64) -> Self {
        ReactNode::Number(value)
    }
}

impl From<bool> for ReactNode {
    fn from(value: bool) -> Self {
        ReactNode::Boolean(value)
    }
}

impl From<ReactElement> for ReactNode {
    fn from(value: ReactElement) -> Self {
        ReactNode::ReactElement(value)
    }
}

impl From<Vec<ReactNode>> for ReactNode {
    fn from(value: Vec<ReactNode>) -> Self {
        ReactNode::Multiple(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
enum ButtonVariant {
    #[serde(rename = "contained")]
    Contained,
    #[serde(rename = "outlined")]
    Outlined,
    #[serde(rename = "text")]
    Text,
    #[strum(default)]
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "attributes")]
enum ReactElement {
    //TODO: standardise the casing here and transform/map in the FE instead
    h1 {
        children: Box<ReactNode>,
    },
    button {
        text: String,
        onClick: Option<Box<FrontendRequest>>,
    },
    Button {
        variant: ButtonVariant,
        children: Box<ReactNode>,
        onClick: Option<Box<FrontendRequest>>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method", content = "params", rename_all = "snake_case")]
enum FrontendRequest {
    Noop,
    SayHello,
    RunMethod { request: Box<FrontendRequest> },
    FetchNext { path: String },
    AppendToBody { component: ReactNode },
}

async fn init() -> Json<FrontendRequest> {
    //TODO: we can define these in some backend yaml files and deserialise rather than hard-code
    let component = ReactNode::Multiple(vec![
        ReactElement::h1 {
            children: "Something".into(),
        }
        .into(),
        ReactElement::Button {
            variant: ButtonVariant::Outlined,
            children: "Click me".into(),
            onClick: Some(FrontendRequest::SayHello.into()),
        }
        .into(),
    ]);

    Json(FrontendRequest::AppendToBody { component })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // build our application with a single route
    let app = Router::new()
        .route("/init", get(init))
        .fallback_service(ServeDir::new("public"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
