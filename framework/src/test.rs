#[macro_export]
macro_rules! standard_tests {
    (
        !no_module
        $parser:ident [
            $($parse_input:literal => $parse_output:expr)*
        ]
        $(
            $part:ident [
                $($input:literal => $output:expr)*
            ]
        )+
    ) => {
        $crate::standard_tests!(@parser, $parser$(, $parse_input => $parse_output)*);
        $(
            $crate::standard_tests!(@part, $parser, $part $(, $input => $output)*);
        )+
    };

    (@parser, $parser:ident) => {};
    (@parser, $parser:ident $(, $input:literal => $output:expr)+) => {
        $crate::paste! {
            #[test]
            fn [<$parser _test>]() -> Result<()> {
                $(
                    assert_eq!($output, $crate::traits::IntoResult::into_result($parser($input))?);
                )+
                Ok(())
            }
        }
    };

    (@part, $parser:ident, $part:ident) => {};
    (@part, $parser:ident, $part:ident $(, $input:literal => $output:expr)+) => {
        $crate::paste! {
            #[test]
            fn [<$part _test>]() -> Result<()> {
                $(
                    let input = $crate::traits::IntoResult::into_result($parser($input))?;
                    let output = $crate::traits::IntoResult::into_result($part(&input))?;
                    assert_eq!($output, output);
                )+
                Ok(())
            }
        }
    };

    (
        $parser:ident [
            $($parse_input:literal => $parse_output:expr)*
        ]
        $(
            $part:ident [
                $($input:literal => $output:expr)*
            ]
        )+
    ) => {
        #[cfg(test)]
        mod test {
            use {super::*, framework::test::*};

            $crate::standard_tests!(!no_module
                $parser [
                    $($parse_input => $parse_output)*
                ]
                $(
                    $part [
                        $($input => $output)*
                    ]
                )+
            );
        }
    };
}
