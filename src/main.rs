trait AgentTrait {
    fn get_name(&self) -> &str;
    fn get_version(&self) -> &str;
}
struct Agent {
    name: String,
    version: String,
}
impl AgentTrait for Agent {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_version(&self) -> &str {
        &self.version
    }
}
fn get_agent(agent: &impl AgentTrait){
    print!("Agent: {}, Version: {}", agent.get_name(), agent.get_version());
}
fn main() {
    let agent = Agent {
        name: "MyAgent".into(),
        version: "1.0.0".into(),
    };
    get_agent(&agent);
}
