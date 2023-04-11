use super::{Frame, Rect, Render};

pub struct Text {
    text: String,
}

impl Render for Text {
    fn render(&self, frame: &mut Frame, region: Rect) {
        todo!()
    }
}
