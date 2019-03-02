//! This module describes a span of text that can have annotations overlaid
//! on top of it.


// text.rs

use std::rc::Rc;
use std::fmt;
use chrono::{DateTime, Utc};
use std::collections::HashMap;


/// Text that can be annotated
pub struct Content {
    pub text: String
}


// Format Content
impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}


/// The types of annotations that can be added to `Content`
#[derive(Debug, PartialEq)]
pub enum AnnotationType {
    Highlight,
    Comment(String),
    Boldface,
    Underline
}


/// An annotation on top of `Content`
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


/// Raw text and a collection of annotations overlaid on top of the text
pub struct AnnotatedContent {
    pub content: Rc<Content>,
    pub annotations: HashMap<usize, Annotation>
}


impl AnnotatedContent {
    pub fn new(text: String) -> AnnotatedContent {
        let content = Rc::new(Content {text});
        let annotations: HashMap<usize, Annotation> = HashMap::new();
        AnnotatedContent {content, annotations}
    }

    pub fn add_annotation(&mut self, a: Annotation) {
        self.annotations.insert(a.start, a);
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

    #[test]
    fn can_create_annotated_content() {
        let text = "Mary had a little lamb.".to_string();
        let ac = AnnotatedContent::new(text);

        assert_eq!(&ac.content.text, "Mary had a little lamb.");
    }

    #[test]
    fn can_add_annotations() {
        let text = "Mary had a little lamb.".to_string();
        let mut ac = AnnotatedContent::new(text);
        let person = Annotation::new(0, 4, AnnotationType::Highlight, &ac.content);

        ac.add_annotation(person);

        assert_eq!(&ac.content.text, "Mary had a little lamb.");
        assert_eq!(ac.annotations.len(), 1usize);
    }
}