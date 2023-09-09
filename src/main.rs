extern crate libui;
use libui::controls::*;
use libui::prelude::*;
struct LibUIWindowData {
    pub title: String,
    pub width: i32,
    pub height: i32,
}
impl LibUIWindowData {
    pub fn new() -> Option<LibUIWindowData> {
        return Some(LibUIWindowData {
            title: String::new(),
            width: 0,
            height: 0,
        });
    }
}
struct LibUIWindow {
    window_data: LibUIWindowData,
}
impl LibUIWindow {
    pub fn new() -> LibUIWindow {
        return LibUIWindow {
            window_data: LibUIWindowData::new().unwrap(),
        };
    }
    pub fn set_window_data(
        &mut self,
        title: &str,
        width: i32,
        height: i32,
    ) -> Result<&LibUIWindow, ()> {
        self.window_data.title = title.to_string();
        self.window_data.width = width;
        self.window_data.height = height;
        return Ok(self);
    }
    pub fn show(&self) -> Result<&LibUIWindow, ()> {
        let mut ui = libui::UI::init().expect("libui UI:initialize error!");
        let mut window = Window::new(
            &ui,
            &self.window_data.title,
            self.window_data.width,
            self.window_data.height,
            WindowType::NoMenubar,
        );
        let mut lay = VerticalBox::new();
        let mut label = Label::new("");
        let mut bt = Button::new("ホリエモン！");
        let mut combox = Combobox::new();
        combox.append("ホリエモン");
        combox.append("ホリエモン2");
        combox.append("ホリエモン3");
        lay.append(combox, LayoutStrategy::Stretchy);
        lay.append(label, LayoutStrategy::Stretchy);
        window.set_child(lay);
        window.show();
        ui.main();

        return Ok(self);
    }
}
fn main() {
    let libwin = LibUIWindow::new()
        .set_window_data("test", 640, 480)
        .unwrap()
        .show();
}
