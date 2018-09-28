
mod parser;

use std::ops::Index;
use hashmap::HashMap;
use nom::Err;
use nom::types::CompleteStr;

/// Instructions are executed in Game.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Instruction {
    /// Jump to given section
    Jump(String),
    /// Talk instruction (textid, Vec<choice's textid, destination section>)
    Talk(String, Vec<(String, String)>),
    /// Special instruction to start buying at a shop
    ShopBuy,
    /// Special instruction to start selling at a shop
    ShopSell,
    /// Special instruction to get locations of dungeons
    GetDungeonLocation,
}

/// Script consists of one or more sections.
/// One section includes one or more instructions.
#[derive(Clone, Serialize, Deserialize)]
pub struct Script(HashMap<String, Vec<Instruction>>);

impl Script {
    pub fn section(&self, s: &str) -> &[Instruction] {
        self.0[s].as_ref()
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptPos {
    pub section: String,
    pub i: usize,
}

impl<'a> Index<&'a ScriptPos> for Script {
    type Output = Instruction;

    fn index(&self, pos: &ScriptPos) -> &Instruction {
        &self.section(&pos.section)[pos.i]
    }
}

/// Object that include script data.
#[derive(Serialize, Deserialize)]
pub struct ScriptObject {
    pub id: String,
    pub script: Script,
}

pub fn parse(input: &str) -> Result<Script, Err<CompleteStr, u32>> {
    self::parser::parse(CompleteStr(input)).map(|result| result.1)
}

