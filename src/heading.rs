use std::cmp::Ordering;

use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Heading {
    level: usize,
    slug: String,
    title: String,
}

pub fn from_html(html: &str) -> Vec<Heading> {
    let re = Regex::new(r#"<h(\d+) id='([^']+)'>(.+)</h\d+>"#).unwrap();
    let mut results = Vec::new();

    for caps in re.captures_iter(&html) {
        let level = caps[1].parse::<usize>().unwrap();
        let slug = caps[2].to_owned();
        let title = caps[3].to_owned();

        results.push(Heading { level, slug, title });
    }

    results
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

        result += &format!(r##"<li><a href="#{}">{}</a>"##, heading.slug, heading.title);
    }

    while last_level > 0 {
        result += "</li>\n</ul>\n";
        last_level -= 1;
    }

    result
}

#[test]
fn from_html_example() {
    let html = "
<h1 id='bla'>Bla</h1>
<h2 id='test'>Test</h2>
<h3 id='testy'>TESTY</h3>
<h3 id='tasty'>tasty</h3>
<h4 id='very'>very</h4>
<h2 id='whatever'>whatever</h2>
<h4 id='test'>Test</h4>";

    assert_eq!(
        from_html(&html),
        [
            Heading {
                level: 1,
                slug: "bla".to_string(),
                title: "Bla".to_string()
            },
            Heading {
                level: 2,
                slug: "test".to_string(),
                title: "Test".to_string()
            },
            Heading {
                level: 3,
                slug: "testy".to_string(),
                title: "TESTY".to_string()
            },
            Heading {
                level: 3,
                slug: "tasty".to_string(),
                title: "tasty".to_string()
            },
            Heading {
                level: 4,
                slug: "very".to_string(),
                title: "very".to_string()
            },
            Heading {
                level: 2,
                slug: "whatever".to_string(),
                title: "whatever".to_string()
            },
            Heading {
                level: 4,
                slug: "test".to_string(),
                title: "Test".to_string()
            },
        ]
    );
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
            slug: "coffee".to_string(),
            title: "Coffee".to_string(),
        },
        Heading {
            level: 1,
            slug: "tea".to_string(),
            title: "Tea".to_string(),
        },
        Heading {
            level: 2,
            slug: "black_tea".to_string(),
            title: "Black tea".to_string(),
        },
        Heading {
            level: 2,
            slug: "green_tea".to_string(),
            title: "Green tea".to_string(),
        },
        Heading {
            level: 1,
            slug: "milk".to_string(),
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
            slug: "coffee".to_string(),
            title: "Coffee".to_string(),
        },
        Heading {
            level: 1,
            slug: "tea".to_string(),
            title: "Tea".to_string(),
        },
        Heading {
            level: 2,
            slug: "black_tea".to_string(),
            title: "Black tea".to_string(),
        },
        Heading {
            level: 2,
            slug: "green_tea".to_string(),
            title: "Green tea".to_string(),
        },
        Heading {
            level: 3,
            slug: "china".to_string(),
            title: "China".to_string(),
        },
        Heading {
            level: 3,
            slug: "africa".to_string(),
            title: "Africa".to_string(),
        },
        Heading {
            level: 1,
            slug: "milk".to_string(),
            title: "Milk".to_string(),
        },
    ];

    let toc = to_html_toc(&headings);
    assert_eq!(toc, expected);
}
