//This module describes a span of text that may or may not have associated
//annotations. For example, a sentence can have multiple REGION_CLASS annotations
//and a token can have a POS annotation.

use std::string::ToString;
use std::rc::Rc;


// todo: There can be annotations on top of more than just text
// The text of a document goes here. The content struct owns the
// text of the document. Anything else that uses this text will
// have to borrow only.
pub struct Content {
    pub text: String
}

impl ToString for Content {
    fn to_string(&self) -> String {
        format!("{}", self.text)
    }
}

// an annotation on the text
pub struct Annotation {
    pub start: usize,
    pub length: usize,
    pub name: String, // The kind of annotation, e.g. automobile
    pub label: String, // The annotation label, e.g. BMW
    pub (crate) parent: Rc<Content>// The `Content` on which this annotation is overlaid
}

impl Annotation {
    pub fn text(&self) -> &str {
        &self.parent.text[self.start..(self.start + self.length)]
    }

    pub fn new(start: usize, length: usize, name: String, label: String, parent: &Rc<Content>) -> Annotation {
        Annotation {start, length, name, label, parent: Rc::clone(parent)}
    }
}

impl ToString for Annotation {
    fn to_string(&self) -> String {
        format!("Annotation(text: {}, name: {}, label: {})", self.text(), self.name, self.label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_annotations() {
        let text = "Destiny 2 is a video game on Microsoft Xbox.".to_string();
        let content = Rc::new(Content {text});

        let annotation1 = Annotation::new(29, 14, "NER".to_string(), "business_name".to_string(), &content);
        let annotation2 = Annotation::new(0, 9, "NER".to_string(), "video_game".to_string(), &content);

        assert_eq!(annotation1.text(), "Microsoft Xbox");
        assert_eq!(annotation2.text(), "Destiny 2");
    }
}
