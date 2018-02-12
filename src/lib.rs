mod annotations;

use annotations::*;
use std::collections::HashMap;
use std::rc::Rc;


//An annotation can be retreived using the starting index of the text onto which
//it's overlaid.
struct AnnotatedContent {
    content: Rc<Content>,
    annotations: HashMap<usize, Annotation>
}

impl AnnotatedContent {
    fn new(text: String) -> AnnotatedContent {
        let content = Rc::new(Content {text});
        let annotations: HashMap<usize, Annotation> = HashMap::new();
        AnnotatedContent {content, annotations}
    }

    fn add_annotation(&mut self, a: Annotation) {
        self.annotations.insert(a.start, a);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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