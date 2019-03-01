//! This module describes a span of text that can have annotations overlaid
//! on top of it.

use std::rc::Rc;
use std::fmt;
use chrono::{DateTime, Utc};


/// Raw text content that can be annotated
pub struct Content {
    pub text: String
}

// Writes Content to stdout
impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}


/// The types of annotations that can be added on top of the text in `Content`
#[derive(Debug, PartialEq)]
pub enum AnnotationType {
    Highlight,
    Comment(String),
    Boldface,
    Underline
}


/// An annotation on top of the text in `Content`
pub struct Annotation {
    /// Index where the `Annotation` starts in `Content.text`.
    pub start: usize,
    // todo: lookup whether `length` is number of characters or what in a `String`
    /// Length of the `Annotation`
    pub length: usize,
    pub overlay: AnnotationType,
    /// Time at which the `Annotation` was created
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

// Writes an Annotation to stdout
impl fmt::Display for Annotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Annotation(text: {}, overlay: {:?}, timestamp: {})", self.text(), self.overlay, self.timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_highlight_annotations() {
        let text = String::from("Destiny 2 is a video game on Microsoft Xbox.");
        let content = Rc::new(Content {text});

        let annotation1 = Annotation::new(29, 14, AnnotationType::Highlight, &content);
        let annotation2 = Annotation::new(0, 9, AnnotationType::Highlight, &content);

        assert_eq!(annotation1.text(), "Microsoft Xbox");
        assert_eq!(annotation2.text(), "Destiny 2");
        assert_eq!(annotation1.overlay, AnnotationType::Highlight);
    }

    #[test]
    fn can_create_comment_annotations() {
        let text = String::from("Destiny 2 is a video game on Microsoft Xbox.");
        let content = Rc::new(Content {text});

        let md1 = AnnotationType::Comment(String::from("The best gaming system."));
        let md2 = AnnotationType::Comment(String::from("The best first-person shooter, to date."));

        let annotation1 = Annotation::new(29, 14, md1, &content);
        let annotation2 = Annotation::new(0, 9, md2, &content);

        assert_eq!(annotation1.text(), "Microsoft Xbox");
        assert_eq!(annotation2.text(), "Destiny 2");
        assert_eq!(annotation1.overlay, AnnotationType::Comment(String::from("The best gaming system.")));
        assert_eq!(annotation2.overlay, AnnotationType::Comment(String::from("The best first-person shooter, to date.")));
    }
}