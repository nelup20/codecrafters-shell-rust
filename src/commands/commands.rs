pub enum Commands {
    Pwd,
    Echo,
    Exit,
    Type,
    Cd,
    Other(String)
}

impl Commands {
    pub fn from_str(input: &str) -> Self {
        match input {
            "pwd" => Self::Pwd,
            "echo" => Self::Echo,
            "exit" => Self::Exit,
            "type" => Self::Type,
            "cd" => Self::Cd,
            _ => Self::Other(String::from(input))
        }
    }
    
    pub fn as_str(&self) -> &str {
        match self {
            Commands::Pwd => "pwd",
            Commands::Echo => "echo",
            Commands::Exit => "exit",
            Commands::Type => "type",
            Commands::Cd => "cd",
            Commands::Other(cmd) => cmd
        }
    }
}