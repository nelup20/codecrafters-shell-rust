pub enum BuiltinCommands {
    Pwd,
    Echo,
    Exit,
    Type,
    Cd,
    History
}

impl BuiltinCommands {
    pub fn from_str(input: &str) -> Self {
        match input {
            "pwd" => Self::Pwd,
            "echo" => Self::Echo,
            "exit" => Self::Exit,
            "type" => Self::Type,
            "cd" => Self::Cd,
            "history" => Self::History,
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
            BuiltinCommands::History => "history",
        }
    }
}
