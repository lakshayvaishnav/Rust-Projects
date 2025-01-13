use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    // this pragraph will have multiple collection of paragraph struct above.
    paragraph: Vec<Paragraph>,
}

fn main() {
    let json = r#"
        {
            "article":"how to work with json in rust",
            "author":"lxsh",
            "paragraph":[
                    {
                        "name":"starting the article"
                    },
                    {
                        "name":"body of the paragraph"
                    },
                    {
                        "name":"lenght of the pargraph"
                    }

            ]
        }"#;

    let result: Article = read_json_typed(json);
    println!(
        "\n\n The name of the paragraph is : {}",
        result.paragraph[0].name
    );
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}
