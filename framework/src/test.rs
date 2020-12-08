#[macro_export]
macro_rules! standard_tests {
    (
        !no_module
        $parser:ident [
            $($parse_input:expr => $parse_output:expr)*
        ]
        $(
            $part:ident [
                $($input:expr => $output:expr)*
            ]
        )+
    ) => {
        $crate::standard_tests!(@parser, $parser$(, $parse_input => $parse_output)*);
        $(
            $crate::standard_tests!(@part, $parser, $part $(, $input => $output)*);
        )+
    };

    (@parser, $parser:ident) => {};
    (@parser, $parser:ident $(, $input:expr => $output:expr)+) => {
        $crate::paste! {
            #[test]
            fn [<$parser _test>]() -> Result<()> {
                $(
                    let result = $crate::traits::IntoResult::into_result($parser($input))?;
                    assert_eq!(result, $output);
                )+
                Ok(())
            }
        }
    };

    (@part, $parser:ident, $part:ident) => {};
    (@part, $parser:ident, $part:ident $(, $input:expr => $output:expr)+) => {
        $crate::paste! {
            #[test]
            fn [<$part _test>]() -> Result<()> {
                $(
                    let input = $crate::traits::IntoResult::into_result($parser($input))?;
                    let output = $crate::traits::IntoResult::into_result($part(&input))?;
                    assert_eq!(output, $output);
                )+
                Ok(())
            }
        }
    };

    (
        $parser:ident [
            $($parse_input:expr => $parse_output:expr)*
        ]
        $(
            $part:ident [
                $($input:expr => $output:expr)*
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
