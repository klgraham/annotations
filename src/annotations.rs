/// This module describes a span of text that can have annotations
/// overlaid on top of it.

use std::string::ToString;
use std::rc::Rc;


/// Raw text content that can be annotated
pub struct Content {
    pub text: String
}

impl ToString for Content {
    fn to_string(&self) -> String {
        format!("{}", self.text)
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
    /// The `Content` on which this annotation is overlaid
    parent: Rc<Content>
}

impl Annotation {
    /// Gets the span of text that is being annotated.
    pub fn text(&self) -> &str {
        &self.parent.text[self.start..(self.start + self.length)]
    }

    pub fn new(start: usize, length: usize, metadata: AnnotationType, parent: &Rc<Content>) -> Annotation {
        Annotation {start, length, overlay: metadata, parent: Rc::clone(parent)}
    }
}

impl ToString for Annotation {
    fn to_string(&self) -> String {
        format!("Annotation(text: {}, metadata: {:?})", self.text(), self.overlay)
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
    }
}
