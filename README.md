# handlebars-js-issue-sample

Related to [this issue](https://github.com/sunng87/handlebars-rust/issues/592)

## Issue Description

I have found a case where the library behaves differently than handlebars.js:

Here is the setup:

### Template

```handlebars
# {{data.name}}

{{#each data.related as |related|}}
{{#with (lookup ../other.contents @key) as | other_related |}}
## [{{other_related.name}}](/{{@key}})
{{related.relationship}}
{{/with}}
{{/each}}
```

### Data

```json
{
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
}
```

### Expected output

```md
# John

## [Mary](/mary)
Siblings
```

### Actual output

```md
# John

## [Mary](/)
Siblings
```

This is a link to the handlebars' playground showing what I am attempting to do: [here][playground-link].

This is a link to a repository that can be cloned to reproduce the issue: [here][repository-link].

## Further notes

The main issue seems to be referencing `@key` inside a `{{#with}}` block.

Do you have any insights on why this is the case, if there is a setting I overlooked in the library, or if there are workarounds?


I am more than happy to help, albeit a little ~~bad~~ inexperienced with rust.

Thanks in advance!


[repository-link]: https://github.com/HectorCastelli/handlebars-js-issue-sample
[playground-link]:https://handlebarsjs.com/playground.html#format=1&currentExample=%7B%22template%22%3A%22%23%20%7B%7Bdata.name%7D%7D%5Cn%5Cn%7B%7B%23each%20data.related%20as%20%7Crelated%7C%7D%7D%5Cn%7B%7B%23with%20(lookup%20..%2Fother.contents%20%40key)%20as%20%7C%20other_related%20%7C%7D%7D%5Cn%23%23%20%5B%7B%7Bother_related.name%7D%7D%5D(%2F%7B%7B%40key%7D%7D)%5Cn%7B%7Brelated.relationship%7D%7D%5Cn%7B%7B%2Fwith%7D%7D%5Cn%7B%7B%2Feach%7D%7D%22%2C%22partials%22%3A%5B%5D%2C%22input%22%3A%22%7B%5Cn%20%20%20%20%5C%22data%5C%22%3A%20%7B%5Cn%20%20%20%20%20%20%20%20%5C%22name%5C%22%3A%20%5C%22John%5C%22%2C%5Cn%20%20%20%20%20%20%20%20%5C%22related%5C%22%3A%20%7B%5Cn%20%20%20%20%20%20%20%20%20%20%20%20%5C%22mary%5C%22%3A%20%7B%5Cn%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5C%22relationship%5C%22%3A%20%5C%22Siblings%5C%22%5Cn%20%20%20%20%20%20%20%20%20%20%20%20%7D%5Cn%20%20%20%20%20%20%20%20%7D%5Cn%20%20%20%20%7D%2C%5Cn%20%20%20%20%5C%22other%5C%22%3A%20%7B%5Cn%20%20%20%20%20%20%20%20%5C%22contents%5C%22%3A%20%7B%5Cn%20%20%20%20%20%20%20%20%20%20%20%20%5C%22john%5C%22%3A%20%7B%5Cn%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5C%22name%5C%22%3A%20%5C%22John%5C%22%2C%5Cn%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5C%22related%5C%22%3A%20%7B%5Cn%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5C%22mary%5C%22%3A%20%7B%5Cn%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5C%22relationship%5C%22%3A%20%5C%22Siblings%5C%22%5Cn%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%7D%5Cn%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%7D%5Cn%20%20%20%20%20%20%20%20%20%20%20%20%7D%2C%5Cn%20%20%20%20%20%20%20%20%20%20%20%20%5C%22mary%5C%22%3A%20%7B%5Cn%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5C%22name%5C%22%3A%20%5C%22Mary%5C%22%5Cn%20%20%20%20%20%20%20%20%20%20%20%20%7D%5Cn%20%20%20%20%20%20%20%20%7D%5Cn%20%20%20%20%7D%5Cn%7D%22%2C%22output%22%3A%22%23%20John%5Cn%5Cn%23%23%20%5BMary%5D(%2Fmary)%5CnSiblings%5Cn%22%2C%22preparationScript%22%3A%22%2F%2F%20Handlebars.registerHelper('loud'%2C%20function(string)%20%7B%5Cn%2F%2F%20%20%20%20return%20string.toUpperCase()%5Cn%2F%2F%20%7D)%3B%5Cn%22%2C%22handlebarsVersion%22%3A%224.7.7%22%7D