import csv
import sys
import os
import math
from typing import Dict, NamedTuple, List, Generator, Tuple, Optional, Any
import matplotlib.pyplot as plt
from matplotlib import ticker

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

class Collection(NamedTuple):
    key: str
    name: str
    alt_key: Optional[str] = None

    def display_key(self) -> str:
        if self.alt_key is None:
            return self.key
        return self.alt_key

class Operation(NamedTuple):
    key: str
    name: str

class Mode(NamedTuple):
    key: str
    name: str

def format_float(number: float) -> str:
    return f'{number:.3f}'.strip('.0')

def size_formatter(sizes: List[int]) -> Any:
    @ticker.FuncFormatter
    def impl(index: int, pos: Any) -> str:
        size = sizes[index]
        if size < 1024 ** 1:
            return f'{round(size)} B'
        if size < 1024 ** 2:
            return f'{format_float(size / 1024 ** 1)} KiB'
        if size < 1024 ** 3:
            return f'{format_float(size / 1024 ** 2)} MiB'
        if size < 1024 ** 4:
            return f'{format_float(size / 1024 ** 3)} GiB'
        return f'{format_float(size / 1024 ** 4)} TiB'
    return impl

@ticker.FuncFormatter
def time_formatter(nanoseconds: int, pos: Any) -> str:
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

class SizeTimeChart(NamedTuple):
    name: str
    title: str
    sizes: List[int]
    times: Dict[str, List[int]]
    ybase: float = 2

    def plot(self, output_dir: str):
        fig, ax = plt.subplots()
        for label, times_list in self.times.items():
            ax.semilogy(
                    range(len(self.sizes)),
                    times_list,
                    label=label,
                    base=self.ybase)
        ax.set_xlabel('input size (log)')
        ax.xaxis.set_major_formatter(size_formatter(self.sizes))
        ax.yaxis.set_major_formatter(time_formatter)
        ax.set_ylabel(f'time (log)')
        ax.set_title(self.title, pad=18.0)
        ax.yaxis.grid(True)
        max_time = max(map(lambda lst: max(lst), self.times.values()))
        min_time = min(map(lambda lst: min(lst), self.times.values()))
        yticks = 10
        log_min_time = math.log(min_time, self.ybase)
        log_max_time = math.log(max_time, self.ybase)
        interval = (log_max_time  - log_min_time) / (yticks - 1)
        ax.set_yticks([
            self.ybase ** (log_min_time + i * interval)
            for i in range(yticks)])
        ax.set_xticks(range(len(self.sizes)))
        ax.legend()
        fig.set_figwidth(8)
        fig.set_figheight(8)
        fig.savefig(
                os.path.join(output_dir, f'{self.name}.png'),
                bbox_inches='tight') 
        plt.close(fig)

def make_collections_chart(
        name: str,
        rows: List[Row],
        title: str,
        mode: Mode,
        operation: Operation,
        collections: List[Collection]) -> SizeTimeChart:
    
    sizes: List[int] = []
    collections_map: Dict[str, Collection] = dict(map(
        lambda col: (col.key, col), collections))
    sizes_per_collection: Dict[str, List[int]] = dict(map(
        lambda col: (col.display_key(), []),
        collections))
    times: Dict[str, List[int]] = dict(map(
        lambda col: (col.display_key(), []),
        collections))

    for row in rows:
        if row.mode == mode.key and row.operation == operation.key:
            if row.collection in collections_map:
                collection_key = collections_map[row.collection].display_key()
                sizes_per_collection[collection_key].append(row.size)
                if row.size not in sizes:
                    sizes.append(row.size)
                times[collection_key].append(row.nanoseconds)

    for current in sizes_per_collection.values():
        assert current == sizes

    return SizeTimeChart(name=name, title=title, sizes=sizes, times=times)

def make_modes_chart(
        name: str,
        rows: List[Row],
        title: str,
        modes: List[Mode],
        operation: Operation,
        collection: Collection) -> SizeTimeChart:
    sizes: List[int] = []
    modes_map: Dict[str, Mode] = dict(map(lambda mode: (mode.key, mode), modes))
    sizes_per_mode: Dict[str, List[int]] = dict(map(
        lambda mode: (mode.key, []), modes))
    times: Dict[str, List[int]] = dict(map(
        lambda mode: (mode.key, []), modes))

    for row in rows:
        if row.collection == collection.key and row.operation == operation.key:
            if row.mode in modes_map:
                sizes_per_mode[row.mode].append(row.size)
                if row.size not in sizes:
                    sizes.append(row.size)
                times[row.mode].append(row.nanoseconds)

    for current in sizes_per_mode.values():
        assert current == sizes

    return SizeTimeChart(name=name, title=title, sizes=sizes, times=times)

def make_charts(rows: List[Row]) -> List[SizeTimeChart]:
    charts = []
    modes = [
            Mode(key='debug', name='debug (no optimizations)'),
            Mode(key='release', name='release (optimized)')]
    operations = [
            Operation(key='create', name='creation'),
            Operation(key='find', name='search for element'),
            Operation(
                key='inc-less-than',
                name='increment all elements smaller than a certain value') ]

    create_collections = [
            Collection(
                key='sorted-array',
                name='sorted array'),
            Collection(
                key='good-local-array',
                name='unsorted array',
                alt_key='unsorted-array'),
            Collection(
                key='linked-list',
                name='linked list'),
            Collection(
                key='with-order-tree',
                name='binary tree',
                alt_key='tree')]

    other_op_collections = [
            Collection(
                key='sorted-array',
                name='sorted array'),
            Collection(
                key='good-local-array',
                name='good locality array'),
            Collection(
                key='bad-local-array',
                name='bad locality array'),
            Collection(
                key='worse-local-array',
                name='worse locality array'),
            Collection(
                key='linked-list',
                name='linked list'),
            Collection(
                key='with-order-tree',
                name='binary tree assuming correct order'),
            Collection(
                key='without-order-tree',
                name='binary tree not assuming correct order')]

    for operation in operations:
        if operation.key == 'create':
            collections = create_collections
        else:
            collections = other_op_collections

        for mode in modes:
            name = f'collections-{operation.key}-{mode.key}'
            title = (f'Operation "{operation.name}" in "{mode.name}"'
                    + ' mode for all collections')
            charts.append(make_collections_chart(
                name,
                rows,
                title,
                mode,
                operation,
                collections))

        for collection in collections:
            name = f'modes-{operation.key}-{collection.display_key()}'
            title = (f'Operation "{operation.name}" with "{collection.name}"'
                    + f' collection for all modes')
            charts.append(make_modes_chart(
                name,
                rows,
                title,
                modes,
                operation,
                collection))
            
    return charts

def plot_charts(output_dir: str, charts: List[SizeTimeChart]):
    os.makedirs(output_dir, exist_ok=True)
    for chart in charts:
        chart.plot(output_dir)

def main():
    if len(sys.argv) != 3:
        print('Usage: main.py [INPUT FILE] [OUTPUT DIR]', file=sys.stderr)
        sys.exit(1)
    rows = parse_rows(sys.argv[1])
    charts = make_charts(rows)
    plot_charts(sys.argv[2], charts)

if __name__ == '__main__':
    main()
