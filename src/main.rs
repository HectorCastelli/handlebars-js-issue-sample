use std::error::Error;

use handlebars::Handlebars;
use serde_json::json;

fn main() -> Result<(), Box<dyn Error>> {
    let mut handlebars = Handlebars::new();

    let data = json!({
        "data": {
            "name": "John",
            "related": {
                "mary": {
                    "relationship": "Siblings"
                }
            }
        },
        "other": {
            "contents": {
                "john": {
                    "name": "John",
                    "related": {
                        "mary": {
                            "relationship": "Siblings"
                        }
                    }
                },
                "mary": {
                    "name": "Mary"
                }
            }
        }
    });

    let template = r#"
        # {{data.name}}

        {{#each data.related as |related|}}
        {{#with (lookup ../other.contents @key) as | other_related |}}
        ## [{{other_related.name}}](/{{@key}})
        {{related.relationship}}
        {{/with}}
        {{/each}}
        "#;

    println!(
        "{}",
        handlebars.render_template(template, &data)?);

    Ok(())
}
