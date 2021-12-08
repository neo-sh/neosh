/// Command is what user sends. Consists of [CommandUnit]
pub struct Command(Vec<CommandUnit>);

// TODO: Operator as an enum
/// Part of a [Command]. Can be either an [Instruction] or an Operator
pub enum CommandUnit {
    Instruction(Instruction),
    Operator(String),
}

/// Instruction is a something that is called with [InstructionUnit]
pub struct Instruction {
    callable: String,
    children: Vec<InstructionUnit>,
}

/// Either an argument or a [Flag]
pub enum InstructionUnit {
    Arg(String),
    Flag(Flag),
}

/// Flag parameters
///
/// name - what comes after `--` or `-`
/// value - what we pass to the flag
/// length - see [FlagLength]
/// storage - see [FlagStorage]
pub struct Flag {
    name: String,
    value: Option<String>,
    length: FlagLength,
    storage: FlagStorage,
}

/// Flag length type
///
/// Short - `-`
/// Long - `--`
pub enum FlagLength {
    Short,
    Long,
}

/// How flag storages it's value
// Inner - --a=b
// Outer - --a b
pub enum FlagStorage {
    Inner,
    Outer,
}

