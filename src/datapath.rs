pub trait Datapath {
    fn execute_instruction(&mut self);
    fn execute_stage(&mut self);
    fn get_register(&self, register: &str) -> Option<u64>;
}
