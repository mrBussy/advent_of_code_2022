use crate::exercises::Exercise;
use crate::utils::macros::{generate_part, generate_part1, generate_part2};
use std::result::Result;
use log::debug;

generate_part1!(
    {
        //Content
        debug!("execute day 1 part 1");
        Ok(String::from("1"))
    },
    "1"
);
generate_part2!(
    {
        //Content
        debug!("execute day 1 part 2");
        Ok(String::new())
    },
    ""
);
