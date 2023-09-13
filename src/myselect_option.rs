struct MySelectedOptionData {
    text: Vec<String>,
}
impl MySelectedOptionData {
    fn new() -> Option<MySelectedOptionData> {
        return Some(MySelectedOptionData { text: Vec::new() });
    }
}
struct MySelectedOption {
    option_data: MySelectedOptionData,
}

impl MySelectedOption {
    fn new(select_all_num: i32) -> Option<MySelectedOption> {
        return Some(MySelectedOption {
            option_data: MySelectedOptionData::new().unwrap(),
        });
    }
    fn append(&mut self, select_text: &str) -> bool {
        self.option_data.text.push(select_text.to_string());
        for val in self.option_data.text.iter() {}
        return true;
    }
    fn select_delete(&mut self, select_index: i32) -> bool {
        return true;
    }
    fn all_delete(&mut self) -> bool {
        return true;
    }
}