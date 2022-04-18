mod array;
mod linked_list;
mod tree;
mod collection;

use clap::Parser;
use collection::Collection;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::{
    error::Error,
    fmt,
    fs,
    io,
    mem,
    path::PathBuf,
    process,
    str::FromStr,
};

type Element = u64;

const ELEMS_IN_PAGE: usize = 0x1000 / mem::size_of::<Element>();
const SIZES: [usize; 9] = [
    ELEMS_IN_PAGE / 4usize.pow(3),
    ELEMS_IN_PAGE / 4usize.pow(2),
    ELEMS_IN_PAGE / 4usize.pow(1),
    ELEMS_IN_PAGE * 4usize.pow(0),
    ELEMS_IN_PAGE * 4usize.pow(1),
    ELEMS_IN_PAGE * 4usize.pow(2),
    ELEMS_IN_PAGE * 4usize.pow(3),
    ELEMS_IN_PAGE * 4usize.pow(4),
    ELEMS_IN_PAGE * 4usize.pow(5),
];

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

#[derive(Debug, Clone)]
struct Collections {
    good_local_array: collection::GoodLocalArray,
    bad_local_array: collection::BadLocalArray,
    worse_local_array: collection::WorseLocalArray,
    sorted_array: collection::SortedArray,
    linked_list: collection::LinkedList,
    with_order_tree: collection::WithOrderTree,
    without_order_tree: collection::WithoutOrderTree,
}

fn main() {
    let arguments = Arguments::parse();
    if let Err(error) = try_main(&arguments) {
        eprintln!("Error: {}", error);
        process::exit(1);
    }
}

fn try_main(arguments: &Arguments) -> io::Result<()> {
    let mut rng = StdRng::from_seed(arguments.seed.bytes);
    let file = fs::OpenOptions::new()
        .create(true)
        .read(false)
        .write(true)
        .truncate(arguments.truncate)
        .append(!arguments.truncate)
        .open(&arguments.output)?;

    let mut csv_writer =
        csv::WriterBuilder::new().has_headers(false).from_writer(file);

    for size in SIZES {
        run_for_size(size, &arguments.mode_name, &mut rng, &mut csv_writer)?;
    }

    Ok(())
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

    let mut collections = run_creation(&elements, mode_name, csv_writer)?;
    let extra_element = rng.gen();
    run_inc_less_than(
        &mut collections,
        &elements,
        extra_element,
        mode_name,
        csv_writer,
    )?;
    run_find(&collections, &elements, extra_element, mode_name, csv_writer)?;

    Ok(())
}

fn run_creation<W>(
    elements: &[Element],
    mode_name: &str,
    csv_writer: &mut csv::Writer<W>,
) -> io::Result<Collections>
where
    W: io::Write,
{
    let oper_name = "create";

    let collections = Collections {
        good_local_array: collection::GoodLocalArray::record_create(
            elements, mode_name, oper_name, csv_writer,
        )?,
        bad_local_array: collection::BadLocalArray::record_create(
            elements, mode_name, oper_name, csv_writer,
        )?,
        worse_local_array: collection::WorseLocalArray::record_create(
            elements, mode_name, oper_name, csv_writer,
        )?,
        sorted_array: collection::SortedArray::record_create(
            elements, mode_name, oper_name, csv_writer,
        )?,
        linked_list: collection::LinkedList::record_create(
            elements, mode_name, oper_name, csv_writer,
        )?,
        with_order_tree: collection::WithOrderTree::record_create(
            elements, mode_name, oper_name, csv_writer,
        )?,
        without_order_tree: collection::WithoutOrderTree::record_create(
            elements, mode_name, oper_name, csv_writer,
        )?,
    };

    Ok(collections)
}

fn run_find<W>(
    collections: &Collections,
    all_elements: &[Element],
    extra_element: Element,
    mode_name: &str,
    csv_writer: &mut csv::Writer<W>,
) -> io::Result<()>
where
    W: io::Write,
{
    let oper_name = "find";

    let target_elements = [
        all_elements[(all_elements.len() / 4).saturating_sub(2)],
        all_elements[(all_elements.len() / 2).saturating_sub(2)],
        all_elements[(3 * all_elements.len() / 4).saturating_sub(2)],
        extra_element,
    ];

    let mut found_all = true;

    found_all &= collections.good_local_array.record_find(
        &target_elements,
        all_elements,
        mode_name,
        oper_name,
        csv_writer,
    )?;

    found_all &= collections.bad_local_array.record_find(
        &target_elements,
        all_elements,
        mode_name,
        oper_name,
        csv_writer,
    )?;

    found_all &= collections.worse_local_array.record_find(
        &target_elements,
        all_elements,
        mode_name,
        oper_name,
        csv_writer,
    )?;

    found_all &= collections.sorted_array.record_find(
        &target_elements,
        all_elements,
        mode_name,
        oper_name,
        csv_writer,
    )?;

    found_all &= collections.with_order_tree.record_find(
        &target_elements,
        all_elements,
        mode_name,
        oper_name,
        csv_writer,
    )?;

    found_all &= collections.without_order_tree.record_find(
        &target_elements,
        all_elements,
        mode_name,
        oper_name,
        csv_writer,
    )?;

    found_all &= collections.linked_list.record_find(
        &target_elements,
        all_elements,
        mode_name,
        oper_name,
        csv_writer,
    )?;

    println!("Found all? {:?}", found_all);

    Ok(())
}

fn run_inc_less_than<W>(
    collections: &mut Collections,
    all_elements: &[Element],
    extra_element: Element,
    mode_name: &str,
    csv_writer: &mut csv::Writer<W>,
) -> io::Result<()>
where
    W: io::Write,
{
    let oper_name = "inc-less-than";

    let target_elements = [
        all_elements[(all_elements.len() / 4).saturating_sub(2)],
        all_elements[(all_elements.len() / 2).saturating_sub(2)],
        all_elements[(3 * all_elements.len() / 4).saturating_sub(2)],
        extra_element,
    ];

    collections.good_local_array.record_inc_less_than(
        &target_elements,
        all_elements,
        mode_name,
        oper_name,
        csv_writer,
    )?;

    collections.bad_local_array.record_inc_less_than(
        &target_elements,
        all_elements,
        mode_name,
        oper_name,
        csv_writer,
    )?;

    collections.worse_local_array.record_inc_less_than(
        &target_elements,
        all_elements,
        mode_name,
        oper_name,
        csv_writer,
    )?;

    collections.sorted_array.record_inc_less_than(
        &target_elements,
        all_elements,
        mode_name,
        oper_name,
        csv_writer,
    )?;

    collections.with_order_tree.record_inc_less_than(
        &target_elements,
        all_elements,
        mode_name,
        oper_name,
        csv_writer,
    )?;

    collections.without_order_tree.record_inc_less_than(
        &target_elements,
        all_elements,
        mode_name,
        oper_name,
        csv_writer,
    )?;

    collections.linked_list.record_inc_less_than(
        &target_elements,
        all_elements,
        mode_name,
        oper_name,
        csv_writer,
    )?;

    Ok(())
}
