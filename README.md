# sisop1-trab2

Code for classroom activity "Trabalho 2" for "Sistemas Operacionais I" course
on UFRGS's Computer Science program.

# Run Benchmarks

## Prerequisites
- Rust programming language tools such as Cargo project manager.

## Command
```sh
./record.sh
```

## Output
Results are placed into `output.csv`.

# Setup Python

## Prerequisites
- Python language tools version 3 as default, such as `python` interpreter 
    and `venv` module (virtual environment).

## Command
```sh
. ./setup.sh
```
Note: the first dot is important, you must source the script.

## Output
Virtual environment installed into `analysis/venv` and activated in the current shell.

# Plot Graphs/Charts

## Prerequisits
- Run `Setup` step in this README;
- Run `Run Benchmarks` step in this README.

## Command
```sh
./plot.sh
```

## Output
Resulting graphs/charts are placed into `charts` directory.


# Query Time For a Specific Execution

## Prerequisites
- Run `Setup` step in this README;
- Run `Run Benchmarks` step in this README.

## Command
```sh
./query -m release -s "4 MiB" -o inc-less-than -c sorted-array
```
Or
```sh
./query --mode debug --size "256 B" --operation find --collection linked-list
```
Note: size must be written with a unit of size, either "B", "KiB", "MiB", "GiB", or "TiB",
and when `1024` is reached, the next unit must be used, e.g. write `"1 MiB"` and not `"1024 KiB"`.
Also, you should quote the size because a single space is required.

## Output
Total time for this specific mode, size, operation and collection is printed to the screen/stdout.
