#[macro_export]
macro_rules! main {
    ($($day:ident)+) => {
        $(mod $day;)+

        pub fn main() {
            $crate::run(&[
                $((stringify!($day), $day::DAY_SPEC),)+
            ]);
        }
    };
}

#[macro_export]
macro_rules! day {
    ($day:literal, $parser:ident => $pt1:ident, $pt2:ident) => {
        pub const DAY_SPEC : &'static dyn framework::traits::Day =
            &($day, stringify!($parser), $parser, stringify!($pt1), $pt1, stringify!($pt2), $pt2);
    }
}
