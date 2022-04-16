mod array;
mod linked_list;
mod tree;

use array::Array;
use clap::Parser;
use linked_list::LinkedList;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::{
    collections::HashSet,
    error::Error,
    fmt,
    fs,
    io,
    mem,
    path::PathBuf,
    process,
    str::FromStr,
    time::{Duration, Instant},
};
use tree::Tree;

type Element = u64;

const ELEMS_IN_PAGE: usize = 0x1000 / mem::size_of::<Element>();
const SIZES: [usize; 6] = [
    ELEMS_IN_PAGE / 16,
    ELEMS_IN_PAGE * 16usize.pow(0),
    ELEMS_IN_PAGE * 16usize.pow(1),
    ELEMS_IN_PAGE * 16usize.pow(2),
    ELEMS_IN_PAGE * 16usize.pow(3),
    ELEMS_IN_PAGE * 16usize.pow(4),
];

#[derive(Debug, Clone, Copy)]
enum Operation {
    Create,
    Find,
    IncLessThan,
}

impl Operation {
    fn render(self) -> &'static str {
        match self {
            Operation::Create => "create",
            Operation::Find => "find",
            Operation::IncLessThan => "inc-less-than",
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", self.render())
    }
}

impl serde::Serialize for Operation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.render())
    }
}

#[derive(Debug, Clone, Copy)]
enum Variant {
    SortedArray,
    UnsortedArray,
    GoodLocalArray,
    BadLocalArray,
    WorseLocalArray,
    Tree,
    WithOrderTree,
    WithoutOrderTree,
    LinkedList,
}

impl Variant {
    fn render(self) -> &'static str {
        match self {
            Variant::SortedArray => "sorted-array",
            Variant::UnsortedArray => "unsorted-array",
            Variant::GoodLocalArray => "good-local-array",
            Variant::BadLocalArray => "bad-local-array",
            Variant::WorseLocalArray => "worse-local-array",
            Variant::WithOrderTree => "tree",
            Variant::Tree => "with-order-tree",
            Variant::WithoutOrderTree => "without-order-tree",
            Variant::LinkedList => "linked-list",
        }
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", self.render())
    }
}

impl serde::Serialize for Variant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.render())
    }
}

#[derive(Debug, Clone)]
struct SeedError;

impl fmt::Display for SeedError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "Seeds must contain 1 to 32 hex digits")
    }
}

impl Error for SeedError {}

#[derive(Debug, Clone, Copy)]
struct Seed {
    bytes: [u8; 32],
}

impl FromStr for Seed {
    type Err = SeedError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut this = Self { bytes: [0; 32] };
        let mut current = 0;
        for ch in input.chars().rev() {
            if current >= this.bytes.len() * 2 {
                Err(SeedError)?
            }
            let nibble = if ch.is_ascii_digit() {
                ch as u8 - b'0'
            } else if ch.is_ascii_uppercase() {
                ch as u8 - b'A' + 10
            } else if ch.is_ascii_lowercase() {
                ch as u8 - b'a' + 10
            } else {
                Err(SeedError)?
            };
            this.bytes[current / 2] |= nibble << (4 * current % 2);
            current += 1;
        }
        Ok(this)
    }
}

#[derive(Debug, Parser)]
struct Arguments {
    #[clap(short, long)]
    output: PathBuf,
    #[clap(short, long)]
    mode_name: String,
    #[clap(short, long, default_value = "0")]
    seed: Seed,
    #[clap(short, long)]
    truncate: bool,
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
struct ReportRow<'mode> {
    mode_name: &'mode str,
    size: usize,
    operation: Operation,
    variant: Variant,
    duration: Duration,
}

#[derive(Debug, Clone)]
struct Collections {
    unsorted_array: Array,
    sorted_array: Array,
    linked_list: LinkedList,
    tree: Tree,
}

fn run_creation<W>(
    elements: &[Element],
    mode_name: &str,
    csv_writer: &mut csv::Writer<W>,
) -> io::Result<Collections>
where
    W: io::Write,
{
    let then = Instant::now();
    let mut unsorted_array = Array::empty();
    for &element in elements {
        unsorted_array.append(element);
    }
    let elapsed = then.elapsed();
    let row = ReportRow {
        mode_name,
        size: elements.len(),
        operation: Operation::Create,
        variant: Variant::UnsortedArray,
        duration: elapsed,
    };
    csv_writer.serialize(row)?;

    let then = Instant::now();
    let mut sorted_array = Array::empty();
    for &element in elements {
        sorted_array.append(element);
    }
    sorted_array.sort();
    let elapsed = then.elapsed();
    let row = ReportRow {
        mode_name,
        size: elements.len(),
        operation: Operation::Create,
        variant: Variant::SortedArray,
        duration: elapsed,
    };
    csv_writer.serialize(row)?;

    let then = Instant::now();
    let mut linked_list = LinkedList::empty();
    for &element in elements {
        linked_list.prepend(element);
    }
    let elapsed = then.elapsed();
    let row = ReportRow {
        mode_name,
        size: elements.len(),
        operation: Operation::Create,
        variant: Variant::LinkedList,
        duration: elapsed,
    };
    csv_writer.serialize(row)?;

    let then = Instant::now();
    let mut tree = Tree::empty();
    for &element in elements {
        tree.insert(element);
    }
    let elapsed = then.elapsed();
    let row = ReportRow {
        mode_name,
        size: elements.len(),
        operation: Operation::Create,
        variant: Variant::Tree,
        duration: elapsed,
    };
    csv_writer.serialize(row)?;

    Ok(Collections { unsorted_array, sorted_array, linked_list, tree })
}

fn run_for_size<R, W>(
    size: usize,
    mode_name: &str,
    mut rng: R,
    csv_writer: &mut csv::Writer<W>,
) -> io::Result<()>
where
    R: Rng,
    W: io::Write,
{
    let mut elements: Vec<Element> = vec![0; size];
    rng.fill(&mut elements[..]);

    let collections = run_creation(&elements, mode_name, csv_writer)?;

    Ok(())
}

fn try_main(arguments: &Arguments) -> io::Result<()> {
    let mut rng = StdRng::from_seed(arguments.seed.bytes);
    let file = fs::OpenOptions::new()
        .create(true)
        .truncate(arguments.truncate)
        .append(!arguments.truncate)
        .open(&arguments.output)?;
    let mut csv_writer = csv::Writer::from_writer(file);

    for size in SIZES {
        run_for_size(size, &arguments.mode_name, &mut rng, &mut csv_writer)?;
    }

    Ok(())
}

fn main() {
    let arguments = Arguments::parse();
    if let Err(error) = try_main(&arguments) {
        eprintln!("Error: {}", error);
        process::exit(1);
    }
}
