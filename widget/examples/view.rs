use tuifw_widget::view::{ViewTree, View};
use tuifw_widget::view::panels::{CanvasPanel, CanvasLayout};
use tuifw_widget::view::decorators::{BorderDecorator};

fn main() {
    let screen = unsafe { tuifw_screen::init() }.unwrap();
    let tree = &mut ViewTree::new(screen, |_| ((), |tree| tree));
    CanvasPanel::new(tree, tree.root());
    let view = View::new(tree, tree.root(), |view| ((), view));
    CanvasLayout::new(tree, view);
    BorderDecorator::new(tree, view);
    view.decorator_set(tree, border_decorator_type().tl(), Some(Text {
        value: Bow::Borrowed("╔"), fg: Color::Green, bg: None, attr: Attr::empty()
    }));
    view.decorator_set(tree, border_decorator_type().tr(), Some(Text {
        value: Bow::Borrowed("╗"), fg: Color::Green, bg: None, attr: Attr::empty()
    }));
    view.decorator_set(tree, border_decorator_type().bl(), Some(Text {
        value: Bow::Borrowed("╚"), fg: Color::Green, bg: None, attr: Attr::empty()
    }));
    view.decorator_set(tree, border_decorator_type().br(), Some(Text {
        value: Bow::Borrowed("╝"), fg: Color::Green, bg: None, attr: Attr::empty()
    }));
    view.decorator_set(tree, border_decorator_type().l(), Some(Text {
        value: Bow::Borrowed("║"), fg: Color::Green, bg: None, attr: Attr::empty()
    }));
    view.decorator_set(tree, border_decorator_type().t(), Some(Text {
        value: Bow::Borrowed("═"), fg: Color::Green, bg: None, attr: Attr::empty()
    }));
    view.decorator_set(tree, border_decorator_type().r(), Some(Text {
        value: Bow::Borrowed("║"), fg: Color::Green, bg: None, attr: Attr::empty()
    }));
    view.decorator_set(tree, border_decorator_type().b(), Some(Text {
        value: Bow::Borrowed("═"), fg: Color::Green, bg: None, attr: Attr::empty()
    }));
}
