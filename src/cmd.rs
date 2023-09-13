struct Cmd {}
impl Cmd {
    fn new() -> Option<Cmd> {
        return Some(Cmd {});
    }
    fn cmd_executble(&self) -> i32 {
        return 0;
    }
}
