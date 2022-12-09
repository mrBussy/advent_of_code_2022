use camino::Utf8PathBuf;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};
use aoc_runner_derive::aoc;

/*
Word to start:
I wanted to try out nom. I saw that https://fasterthanli.me/series/advent-of-code-2022/part-7 has a great tutorial.
I used the code from there to learn. The comment in this module therefor are her for my learnings.
 */


// Take a string as input.
// take each character at a time and push that into a new string.
fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}
#[derive(Debug)]
struct Ls;

// Take a string as input. 
// If the start of the string is ls --> Map it to the LS Struct
fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

// Take a string as input. 
// If the start of the string is ls --> Map it to the CD Struct
// Do not forget: there are params to take into account
fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

// By implementing the From you can restrict one extra wrapping around the command
impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

// Try to pars the given line. If one of the command is recognised, then this is mapped using the 2 functions.
// Each command star with a $
fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64),
}

// If the given input starts with a integer then take that as a file _ size
// if is starts with "dir" then use that
fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, _)| Entry::File(size),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}
#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

// Try to parse each line and either have an entry or a command as return
fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}
#[derive(Debug)]
struct FsEntry {
    //path: Utf8PathBuf,
    size: u64,
    children: Vec<FsEntry>,
}

impl FsEntry {
    fn build(mut self, it: &mut dyn Iterator<Item = Line>) -> Self {
        while let Some(line) = it.next() {
            match line {
                Line::Command(Command::Cd(sub)) => match sub.as_str() {
                    "/" => {
                        // muffin,
                    }
                    ".." => break,
                    _ => self.children.push(
                        FsEntry {
                            //path: sub.clone(),
                            size: 0,
                            children: vec![],
                        }
                        .build(it),
                    ),
                },
                Line::Entry(Entry::File(size)) => {
                    self.children.push(FsEntry {
                        //path,
                        size,
                        children: vec![],
                    });
                }
                _ => {
                    // ignore other commands
                }
            }
        }
        self
    }
    fn total_size(&self) -> u64 {
        self.size + self.children.iter().map(|c| c.total_size()).sum::<u64>()
    }

    fn all_dirs(&self) -> Box<dyn Iterator<Item = &FsEntry> + '_> {
        Box::new(
            std::iter::once(self).chain(
                self.children
                    .iter()
                    .filter(|c| !c.children.is_empty())
                    .flat_map(|c| c.all_dirs()),
            ),
        )
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> Option<u64> {

    let mut lines = input
    .lines()
    .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let root = FsEntry {
        //path: "/".into(),
        size: 0,
        children: vec![],
    }
    .build(&mut lines);
    dbg!(&root);

    // solving part 1 because it's the same difficulty as part 2, just less code
    let sum = root
        .all_dirs()
        .map(|d| d.total_size())
        .filter(|&s| s <= 100_000)
        .sum::<u64>();
    dbg!(sum);

    Some(sum)
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> Option<u64> {

    let mut lines = input
    .lines()
    .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let root = FsEntry {
        //path: "/".into(),
        size: 0,
        children: vec![],
    }
    .build(&mut lines);
    dbg!(&root);

    let required_disk_space = 30_000_000;
    let total_disk_space =    70_000_000;
    let unused_space = total_disk_space - root.total_size();
    let required_to_free = required_disk_space - unused_space;

    // solving part 2:
    // Total disk size - Size of / = amount of free space
    let sum = root
        .all_dirs()
        .map(|d| d.total_size())
        .filter(|&s| s >= required_to_free)
        .min();
    dbg!(sum);

    sum
}


#[cfg(test)]
mod tests {

    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(
"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"
        ),
        Some(95437));

    }


    #[test]
    fn test_part2() {
        assert_eq!(part2(
"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"
        ),
        Some(24933642));
    }

}