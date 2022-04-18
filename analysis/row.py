import csv
from typing import Dict, NamedTuple, List, Generator, Tuple, Optional, Any

class BadRowFormat(Exception):
    def __init__(self, row_strings: List[str], message: str):
        super().__init__(
                f'Error on row strings {",".join(row_strings)}: {message}')

class Row(NamedTuple):
    mode: str
    size: int
    operation: str
    collection: str
    nanoseconds: int

    @staticmethod
    def parse(row_strings: List[str]) -> 'Row':
        if len(row_strings) != 5:
            raise BadRowFormat(row_strings, 'must have 5 elements')

        try:
            size = int(row_strings[1])
        except ValueError as error:
            raise BadRowFormat(row_strings, f'size parse error because {error}')
            
        try:
            nanoseconds = int(row_strings[4])
        except ValueError as error:
            raise BadRowFormat(
                row_strings,
                f'nanoseconds parse error because {error}')

        return Row(
            mode=row_strings[0],
            size=size,
            operation=row_strings[2],
            collection=row_strings[3],
            nanoseconds=nanoseconds)

def parse_rows(path: str) -> List[Row]:
    rows: List[Row] = []
    with open(path) as file:
        reader = csv.reader(file)
        for row in reader:
            rows.append(Row.parse(row))
    return rows

def format_float(number: float) -> str:
    return f'{number:.3f}'.strip('.0')

def format_size(size: int) -> str:
    if size < 1024 ** 1:
        return f'{round(size)} B'
    if size < 1024 ** 2:
        return f'{format_float(size / 1024 ** 1)} KiB'
    if size < 1024 ** 3:
        return f'{format_float(size / 1024 ** 2)} MiB'
    if size < 1024 ** 4:
        return f'{format_float(size / 1024 ** 3)} GiB'
    return f'{format_float(size / 1024 ** 4)} TiB'

def format_time(nanoseconds: int) -> str:
    if nanoseconds < 1000 ** 1:
        return f'{round(nanoseconds)} ns'
    if nanoseconds < 1000 ** 2:
        return f'{format_float(nanoseconds / 1000 ** 1)} μs'
    if nanoseconds < 1000 ** 3:
        return f'{format_float(nanoseconds / 1000 ** 2)} ms'
    if nanoseconds < 1000 ** 3 * 60:
        return f'{format_float(nanoseconds / 1000 ** 3)} s'
    if nanoseconds < 1000 ** 3 * 60 * 60:
        return f'{format_float(nanoseconds / (1000 ** 3 * 60))} m'
    return f'{format_float(nanoseconds / (1000 ** 3 * 60 * 60))} h'
