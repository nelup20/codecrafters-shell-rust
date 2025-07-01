pub enum BuiltinCommands {
    Pwd,
    Echo,
    Exit,
    Type,
    Cd,
}

impl BuiltinCommands {
    pub fn from_str(input: &str) -> Self {
        match input {
            "pwd" => Self::Pwd,
            "echo" => Self::Echo,
            "exit" => Self::Exit,
            "type" => Self::Type,
            "cd" => Self::Cd,
            _ => {
                unreachable!()
            }
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            BuiltinCommands::Pwd => "pwd",
            BuiltinCommands::Echo => "echo",
            BuiltinCommands::Exit => "exit",
            BuiltinCommands::Type => "type",
            BuiltinCommands::Cd => "cd",
        }
    }
}
