pub struct Tree {
    pub root: Option<Module>,
}

impl Tree {
    pub fn new() -> Self {
        Self { root: None }
    }
}

pub struct Module {
    pub name: String,
    pub items: Vec<()>,
}
