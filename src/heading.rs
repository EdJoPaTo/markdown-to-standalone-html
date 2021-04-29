use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct Heading {
    pub level: u32,
    pub anchor: String,
    pub title: String,
}

pub fn to_html_toc(headings: &[Heading]) -> String {
    let mut result = String::new();

    let mut last_level = 0;

    for heading in headings {
        match heading.level.cmp(&last_level) {
            Ordering::Greater => {
                while heading.level > last_level {
                    result += "\n<ul>\n";
                    last_level += 1;
                }
            }
            Ordering::Less | Ordering::Equal => {
                while heading.level < last_level {
                    result += "</li>\n</ul>\n";
                    last_level -= 1;
                }
                result += "</li>\n";
            }
        }

        result += &format!(
            r##"<li><a href="#{}">{}</a>"##,
            heading.anchor, heading.title
        );
    }

    while last_level > 0 {
        result += "</li>\n</ul>\n";
        last_level -= 1;
    }

    result
}

#[test]
fn to_html_toc_example_level2() {
    let expected = r##"
<ul>
<li><a href="#coffee">Coffee</a></li>
<li><a href="#tea">Tea</a>
<ul>
<li><a href="#black_tea">Black tea</a></li>
<li><a href="#green_tea">Green tea</a></li>
</ul>
</li>
<li><a href="#milk">Milk</a></li>
</ul>
"##;

    let headings = vec![
        Heading {
            level: 1,
            anchor: "coffee".to_string(),
            title: "Coffee".to_string(),
        },
        Heading {
            level: 1,
            anchor: "tea".to_string(),
            title: "Tea".to_string(),
        },
        Heading {
            level: 2,
            anchor: "black_tea".to_string(),
            title: "Black tea".to_string(),
        },
        Heading {
            level: 2,
            anchor: "green_tea".to_string(),
            title: "Green tea".to_string(),
        },
        Heading {
            level: 1,
            anchor: "milk".to_string(),
            title: "Milk".to_string(),
        },
    ];

    let toc = to_html_toc(&headings);
    assert_eq!(toc, expected);
}

#[test]
fn to_html_toc_example_level3() {
    let expected = r##"
<ul>
<li><a href="#coffee">Coffee</a></li>
<li><a href="#tea">Tea</a>
<ul>
<li><a href="#black_tea">Black tea</a></li>
<li><a href="#green_tea">Green tea</a>
<ul>
<li><a href="#china">China</a></li>
<li><a href="#africa">Africa</a></li>
</ul>
</li>
</ul>
</li>
<li><a href="#milk">Milk</a></li>
</ul>
"##;

    let headings = vec![
        Heading {
            level: 1,
            anchor: "coffee".to_string(),
            title: "Coffee".to_string(),
        },
        Heading {
            level: 1,
            anchor: "tea".to_string(),
            title: "Tea".to_string(),
        },
        Heading {
            level: 2,
            anchor: "black_tea".to_string(),
            title: "Black tea".to_string(),
        },
        Heading {
            level: 2,
            anchor: "green_tea".to_string(),
            title: "Green tea".to_string(),
        },
        Heading {
            level: 3,
            anchor: "china".to_string(),
            title: "China".to_string(),
        },
        Heading {
            level: 3,
            anchor: "africa".to_string(),
            title: "Africa".to_string(),
        },
        Heading {
            level: 1,
            anchor: "milk".to_string(),
            title: "Milk".to_string(),
        },
    ];

    let toc = to_html_toc(&headings);
    assert_eq!(toc, expected);
}
