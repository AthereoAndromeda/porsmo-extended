use crate::{CounterUI, prelude::*};

#[derive(Debug, Default, Clone, Copy)]
pub struct AlarmUI {}

impl CounterUI for AlarmUI {
    fn show(&mut self, out: &mut impl std::io::Write) -> Result<()> {
        todo!()
    }

    fn update(&mut self, command: crate::input::Command) {
        todo!()
    }
}
