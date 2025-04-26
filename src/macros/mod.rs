/// Creates a crossterm `queue` with each item printed in a new line. Mostly to reduce boilerplate
#[macro_export]
macro_rules! new_line_queue {
    ($out:expr $(, $e:expr)+ $(,)?) => {{
        use ::crossterm::{
            cursor::{MoveTo,MoveToNextLine},
            terminal::{Clear, ClearType},
            style::Print,
            queue,
        };

        queue!(
            $out,
            MoveTo(0, 0),
            $(
                Print($e),
                Clear(ClearType::UntilNewLine),
                MoveToNextLine(1),
            )+
            Clear(ClearType::FromCursorDown),
        )
    }};
}
