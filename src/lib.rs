use std::{fs::File, io::Write};

const MY_FILE_THAT_HAS_TO_SHIP: &[u8] = include_bytes!("../this-should-be-in-crate.sql");

pub struct MyApp {
    data: File,
}

impl Default for MyApp {
    fn default() -> Self {
        Self::new()
    }
}

impl MyApp {
    pub fn new() -> Self {
        let mut f = File::create("./shipped.sql").unwrap();
        f.write_all(MY_FILE_THAT_HAS_TO_SHIP).unwrap();
        Self { data: f }
    }

    pub fn sync(&self) {
        self.data.sync_all().unwrap();
        todo!()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
