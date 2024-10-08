use std::cmp::Ordering;
use std::fmt::Write;

use lazy_regex::regex;
use pulldown_cmark::HeadingLevel;

pub struct Heading {
    pub level: HeadingLevel,
    pub anchor: String,
    pub title: String,
}

#[derive(Default)]
pub struct Headings {
    list: Vec<Heading>,
    existing_anchors: Vec<String>,
}

impl Headings {
    /// Returns the anchor to be used in links to this heading
    pub fn add(&mut self, level: HeadingLevel, title: &str) -> String {
        // filter out non ascii
        let main = regex!("[^a-zA-Z\\d]+")
            .replace_all(title, "-")
            .trim_matches('-')
            .to_ascii_lowercase();

        let mut anchor = main.clone();
        let mut index = 1;
        while self.existing_anchors.contains(&anchor) {
            index += 1;
            anchor = format!("{main}-{index}");
        }

        self.existing_anchors.push(anchor.clone());
        self.list.push(Heading {
            level,
            anchor: anchor.clone(),
            title: title.to_owned(),
        });

        anchor
    }

    pub fn finish(self) -> Vec<Heading> {
        self.list
    }
}

pub fn to_html_toc(headings: &[Heading]) -> String {
    let mut result = String::new();

    let mut last_level: u8 = 0;

    for heading in headings {
        let level = match heading.level {
            HeadingLevel::H1 => 1,
            HeadingLevel::H2 => 2,
            HeadingLevel::H3 => 3,
            HeadingLevel::H4 => 4,
            HeadingLevel::H5 => 5,
            HeadingLevel::H6 => 6,
        };

        match level.cmp(&last_level) {
            Ordering::Greater => {
                while level > last_level {
                    result += "\n<ul>\n";
                    last_level += 1;
                }
            }
            Ordering::Less | Ordering::Equal => {
                while level < last_level {
                    result += "</li>\n</ul>\n";
                    last_level -= 1;
                }
                result += "</li>\n";
            }
        }

        write!(
            result,
            r##"<li><a href="#{}">{}</a>"##,
            heading.anchor, heading.title
        )
        .unwrap();
    }

    while last_level > 0 {
        result += "</li>\n</ul>\n";
        last_level -= 1;
    }

    result
}

#[test]
fn anchor_of_title_examples() {
    let mut headings = Headings::default();
    assert_eq!("a-b", headings.add(HeadingLevel::H1, " A b"));
    assert_eq!(
        "passw-rter",
        headings.add(HeadingLevel::H1, "passw\u{f6}rter")
    );
}

#[test]
fn anchor_of_title_is_unique() {
    let mut headings = Headings::default();
    assert_eq!("a", headings.add(HeadingLevel::H1, "a"));
    assert_eq!("a-2", headings.add(HeadingLevel::H1, "a"));
    assert_eq!("a-3", headings.add(HeadingLevel::H1, "a"));
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
            level: HeadingLevel::H1,
            anchor: "coffee".to_owned(),
            title: "Coffee".to_owned(),
        },
        Heading {
            level: HeadingLevel::H1,
            anchor: "tea".to_owned(),
            title: "Tea".to_owned(),
        },
        Heading {
            level: HeadingLevel::H2,
            anchor: "black_tea".to_owned(),
            title: "Black tea".to_owned(),
        },
        Heading {
            level: HeadingLevel::H2,
            anchor: "green_tea".to_owned(),
            title: "Green tea".to_owned(),
        },
        Heading {
            level: HeadingLevel::H1,
            anchor: "milk".to_owned(),
            title: "Milk".to_owned(),
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
            level: HeadingLevel::H1,
            anchor: "coffee".to_owned(),
            title: "Coffee".to_owned(),
        },
        Heading {
            level: HeadingLevel::H1,
            anchor: "tea".to_owned(),
            title: "Tea".to_owned(),
        },
        Heading {
            level: HeadingLevel::H2,
            anchor: "black_tea".to_owned(),
            title: "Black tea".to_owned(),
        },
        Heading {
            level: HeadingLevel::H2,
            anchor: "green_tea".to_owned(),
            title: "Green tea".to_owned(),
        },
        Heading {
            level: HeadingLevel::H3,
            anchor: "china".to_owned(),
            title: "China".to_owned(),
        },
        Heading {
            level: HeadingLevel::H3,
            anchor: "africa".to_owned(),
            title: "Africa".to_owned(),
        },
        Heading {
            level: HeadingLevel::H1,
            anchor: "milk".to_owned(),
            title: "Milk".to_owned(),
        },
    ];

    let toc = to_html_toc(&headings);
    assert_eq!(toc, expected);
}
