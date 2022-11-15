
#[macro_export]
macro_rules! generate_part {
    ($name:ident, $test_name:ident, $content:expr, $test_result:expr) => {
        pub struct $name;
        impl $name {
            pub fn new() -> $name {
                $name {}
            }
            
        }
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
        
        impl Exercise for $name {
            fn execute(&self) -> Result<String, String> {
                $content
            }

            fn get_name(&self) -> Option<String> {
                Some(String::from(stringify!($name)))
            }
        }


        #[cfg(test)]
        mod $test_name {
            use $crate::exercises::Exercise;

            #[test]
            fn part_1() {
                assert_eq!($test_result, super::$name::new().execute().unwrap());
            }
        }
    };
}
#[macro_export]
macro_rules! generate_part1 {
    ($content:expr, $test_result:expr) => {
        generate_part!(Part1, part1_test, $content, $test_result);
    };
}
#[macro_export]
macro_rules! generate_part2 {
    ($content:expr, $test_result:expr) => {
        generate_part!(Part2, part2_test, $content, $test_result);
    };
}

pub(crate) use generate_part; // Now classic paths Just Work™
pub(crate) use generate_part1; // Now classic paths Just Work™
pub(crate) use generate_part2; // Now classic paths Just Work™