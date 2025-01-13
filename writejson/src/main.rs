use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    authror: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let article: Article = Article {
        article: String::from("how to work with json in rust"),
        authror: String::from("akhil"),
        // vector is collection of multiple structs...
        paragraph: vec![
            Paragraph {
                name: String::from("first sentence"),
            },
            Paragraph {
                name: String::from("from body of paragrah"),
            },
            Paragraph {
                name: String::from("string from end of the paragraph"),
            },
        ],
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("the json is : {}", json)
}
