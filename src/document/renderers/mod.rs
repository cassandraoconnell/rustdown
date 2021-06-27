pub mod normal_element;
pub mod text_element;
pub mod void_element;

pub trait Render {
    fn render(&self) -> String;
}
