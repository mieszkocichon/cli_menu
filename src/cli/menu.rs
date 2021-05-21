pub mod menu {

    use std::collections::HashMap;


    pub trait Command {
        fn execute(&self);
    }

    pub mod commands {

        use crate::menu::Command;


        pub struct DrawCommand {}
        impl DrawCommand {
            pub fn new() -> DrawCommand {
                DrawCommand {
                }
            }
        }
        impl Command for DrawCommand {
            fn execute(&self) {
                println!("DrawCommand");
            }
        }
        
        pub struct WriteCommand {}
        impl WriteCommand {
            pub fn new() -> WriteCommand {
                WriteCommand {
                    
                }
            }
        }
        impl Command for WriteCommand {
            fn execute(&self) {
                println!("WriteCommand");
            }
        }
    }

    pub struct Menu {
        string_command_map: HashMap<String, Box<dyn Command>>,
    }
    
    impl Menu {
        pub fn new() -> Self {
            Self {
                string_command_map: HashMap::new()
            }
        }
    
        pub fn initialize(&mut self) {
            self.string_command_map.insert("1".to_string(), Box::new(commands::DrawCommand::new())); 
            self.string_command_map.insert("2".to_string(), Box::new(commands::WriteCommand::new())); 
        }
    
        pub fn execute(self, key: &str) {
            match self.string_command_map.get(key) {
                None => println!("Failed"),
                Some(c) => {
                    c.execute();
                },
            }
        }
    }
}