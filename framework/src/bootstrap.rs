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
        struct DayStruct;
        impl framework::traits::Day for DayStruct {
            fn nr(&self) -> u32 {
                $day
            }

            #[allow(unreachable_code)]
            fn evaluate(
                &self,
                input: $crate::ascii::AString,
            ) -> arrayvec::ArrayVec<[(&'static str, Result<String>); 2]> {
                use framework::traits::{IntoError, IntoResult};
                let mut res = arrayvec::ArrayVec::<[(&'static str, Result<String>); 2]>::new();
                let input = match $parser(&input).into_result() {
                    Ok(v) => v,
                    Err(err) => {
                        res.push((stringify!($parser), Err(err)));
                        return res;
                    }
                };

                res.push((
                    stringify!($pt1),
                    $pt1(&input)
                        .into_result()
                        .map(|x| x.to_string())
                        .map_err(|x| x.into_error()),
                ));
                res.push((
                    stringify!($pt2),
                    $pt2(&input)
                        .into_result()
                        .map(|x| x.to_string())
                        .map_err(|x| x.into_error()),
                ));
                res
            }
        }
        pub const DAY_SPEC: &'static dyn framework::traits::Day = &DayStruct;
    };
}
