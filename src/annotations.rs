/// This module describes a span of text that can have annotations
/// overlaid on top of it.

use std::rc::Rc;
use std::fmt;
use chrono::{DateTime, Utc};


/// Raw text content that can be annotated
pub struct Content {
    pub text: String
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}


/// The types of annotations that can be added on top of the text in `Content`
#[derive(Debug, PartialEq)]
pub enum AnnotationType {
    Highlight,
    Comment(String)
}


// An annotation on top of the text in `Content`
pub struct Annotation {
    pub start: usize,
    pub length: usize,
    pub overlay: AnnotationType,
    pub timestamp: DateTime<Utc>,
    /// The `Content` on which this annotation is overlaid
    parent: Rc<Content>
}

impl Annotation {
    /// Gets the span of text that is being annotated.
    pub fn text(&self) -> &str {
        &self.parent.text[self.start..(self.start + self.length)]
    }

    pub fn new(start: usize, length: usize, overlay: AnnotationType, parent: &Rc<Content>) -> Annotation {
        let timestamp = Utc::now();
        Annotation {start, length, overlay, timestamp, parent: Rc::clone(parent)}
    }
}

impl fmt::Display for Annotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Annotation(text: {}, overlay: {:?}, timestamp: {})", self.text(), self.overlay, self.timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_annotations() {
        let text = String::from("Destiny 2 is a video game on Microsoft Xbox.");
        let content = Rc::new(Content {text});

        let md1 = AnnotationType::Comment(String::from("The best gaming system."));

        let annotation1 = Annotation::new(29, 14, md1, &content);
        let annotation2 = Annotation::new(0, 9, AnnotationType::Highlight, &content);

        assert_eq!(annotation1.text(), "Microsoft Xbox");
        assert_eq!(annotation2.text(), "Destiny 2");
        assert_eq!(annotation1.overlay, AnnotationType::Comment(String::from("The best gaming system.")));
        println!("{}", annotation2);
    }
}
