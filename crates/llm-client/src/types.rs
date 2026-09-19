//! Shared request and response types for model clients.
//! This is an internal protocol:agent core and agent-memory only accept these types.


use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug,Clone,PartialEq,Serialize,Deserialize)]
#[serde(tag = "type",rename_all = "snake_case")]
pub enum Message {
    User(UserMessage),
    Assistant(AssistantMessage),
    AssistantMemory(AssistantMemoryMessage),
    Tool(ToolResultMessage),
}

#[derive(Debug,Clone,PartialEq,Serialize,Deserialize)]
pub struct UserMessage {
    pub context : String,
}
#[derive(Debug,Clone,PartialEq,Serialize,Deserialize)]
pub struct AssistantMessage {
    #[serde(default)]
    pub context : String,
    #[serde(default,skip_serializing_if = "Vec::is_empty")]
    pub tool_calls : Vec<ToolCall>,
    #[serde(default,skip_serializing_if = "Vec::is_empty")]
    pub skill_calls : Vec<SkillCall>,
}
#[derive(Debug,Clone,PartialEq,Serialize,Deserialize)]
pub struct AssistantMemoryMessage {
    #[serde(default)]
    pub context : String,
    #[serde(default,skip_serializing_if = "Vec::is_empty")]
    pub tool_calls : Vec<ToolCall>,
    #[serde(default,skip_serializing_if = "Vec::is_empty")]
    pub memory_updates : Vec<MemoryUpdate>,
    #[serde(default,skip_serializing_if = "Vec::is_empty")]
    pub skill_calls : Vec<SkillCall>,
}
