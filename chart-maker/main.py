import csv
import sys
import os
from typing import Dict, NamedTuple, List, Generator, Tuple, Optional
import matplotlib.pyplot as plt

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

class SizeTimeChart(NamedTuple):
    title: str
    sizes: List[int]
    times: Dict[str, List[int]]

    def plot(self, path: str):
        fig, ax = plt.subplots()
        for label, times_list in self.times.items():
            ax.plot(self.sizes, times_list, label=label)
        ax.set_xlabel('Input size')
        ax.set_ylabel('Time')
        ax.set_title(self.title, pad=18.0)
        ax.yaxis.grid(True)
        max_time = max(map(lambda lst: max(lst), self.times.values()))
        min_time = min(map(lambda lst: min(lst), self.times.values()))
        yticks = 5
        interval = (max_time - min_time) / (yticks - 1);
        ax.set_yticks([
            min_time + i * interval
            for i in range(0, yticks)])
        ax.legend()
        fig.savefig(path, bbox_inches='tight') 
        plt.close(fig)

class Charts(NamedTuple):
    debug_creation: SizeTimeChart
    debug_find: SizeTimeChart
    debug_inc_less_than: SizeTimeChart
    release_creation: SizeTimeChart
    release_find: SizeTimeChart
    release_inc_less_than: SizeTimeChart
    unsorted_array_creation: SizeTimeChart
    sorted_array_creation: SizeTimeChart
    tree_creation: SizeTimeChart
    linked_list_creation: SizeTimeChart
    good_local_array_find: SizeTimeChart
    bad_local_array_find: SizeTimeChart
    worse_local_array_find: SizeTimeChart
    sorted_array_find: SizeTimeChart
    with_order_tree_find: SizeTimeChart
    without_order_tree_find: SizeTimeChart
    linked_list_find: SizeTimeChart
    good_local_array_inc_less_than: SizeTimeChart
    bad_local_array_inc_less_than: SizeTimeChart
    worse_local_array_inc_less_than: SizeTimeChart
    sorted_array_inc_less_than: SizeTimeChart
    with_order_tree_inc_less_than: SizeTimeChart
    without_order_tree_inc_less_than: SizeTimeChart
    linked_list_inc_less_than: SizeTimeChart

    def plot(self, output_dir: str):
        os.makedirs(output_dir, exist_ok=True)
        self.debug_creation.plot(
           os.path.join(output_dir, '.png'))
        self.debug_find.plot(
           os.path.join(output_dir, '.png'))
        self.debug_inc_less_than.plot(
           os.path.join(output_dir, '.png'))
        self.release_creation.plot(
           os.path.join(output_dir, '.png'))
        self.release_find.plot(
           os.path.join(output_dir, '.png'))
        self.release_inc_less_than.plot(
           os.path.join(output_dir, '.png'))
        self.unsorted_array_creation.plot(
           os.path.join(output_dir, '.png'))
        self.sorted_array_creation.plot(
           os.path.join(output_dir, '.png'))
        self.tree_creation.plot(
           os.path.join(output_dir, '.png'))
        self.linked_list_creation.plot(
           os.path.join(output_dir, '.png'))
        self.good_local_array_find.plot(
           os.path.join(output_dir, '.png'))
        self.bad_local_array_find.plot(
           os.path.join(output_dir, '.png'))
        self.worse_local_array_find.plot(
           os.path.join(output_dir, '.png'))
        self.sorted_array_find.plot(
           os.path.join(output_dir, '.png'))
        self.with_order_tree_find.plot(
           os.path.join(output_dir, '.png'))
        self.without_order_tree_find.plot(
           os.path.join(output_dir, '.png'))
        self.linked_list_find.plot(
           os.path.join(output_dir, '.png'))
        self.good_local_array_inc_less_than.plot(
           os.path.join(output_dir, '.png'))
        self.bad_local_array_inc_less_than.plot(
           os.path.join(output_dir, '.png'))
        self.worse_local_array_inc_less_than.plot(
           os.path.join(output_dir, '.png'))
        self.sorted_array_inc_less_than.plot(
           os.path.join(output_dir, '.png'))
        self.with_order_tree_inc_less_than.plot(
           os.path.join(output_dir, '.png'))
        self.without_order_tree_inc_less_than.plot(
           os.path.join(output_dir, '.png'))
        self.linked_list_inc_less_than.plot(
           os.path.join(output_dir, '.png'))

    @staticmethod
    def make_all_collections_chart(
            rows: List[Row],
            title: str,
            mode: str,
            operation: str) -> SizeTimeChart:
        sizes: List[int] = []
        sizes_per_collection: Dict[str, List[int]] = dict()
        times: Dict[str, List[int]] = dict()

        for row in rows:
            if row.mode == mode and row.operation == operation:
                if row.collection not in sizes_per_collection:
                    sizes_per_collection[row.collection] = []
                sizes_per_collection[row.collection].append(row.size)
                if row.size not in sizes:
                    sizes.append(row.size)
                if row.collection not in times:
                    times[row.collection] = []
                times[row.collection].append(row.nanoseconds)

        for current in sizes_per_collection.values():
            assert current == sizes

        return SizeTimeChart(title, sizes, times)

    @staticmethod
    def make_reduced_collections_chart(
            rows: List[Row],
            title: str,
            mode: str,
            operation: str) -> SizeTimeChart:
        sizes: List[int] = []
        sizes_per_collection: Dict[str, List[int]] = dict()
        times: Dict[str, List[int]] = dict()

        for row in rows:
            if row.mode == mode and row.operation == operation:
                collection = row.collection
                if collection in [
                        'bad-local-array',
                        'worse-local-array',
                        'without-order-tree']:
                    continue
                if collection == 'good-local-array':
                    collection = 'unsorted-array'
                if collection == 'with-order-tree':
                    collection = 'tree'
                if collection not in sizes_per_collection:
                    sizes_per_collection[collection] = []
                sizes_per_collection[collection].append(row.size)
                if row.size not in sizes:
                    sizes.append(row.size)
                if collection not in times:
                    times[collection] = []
                times[collection].append(row.nanoseconds)

        for current in sizes_per_collection.values():
            assert current == sizes

        return SizeTimeChart(title, sizes, times)

    @staticmethod
    def make_mode_chart(
            rows: List[Row],
            title: str,
            collection: str,
            operation: str,
            rename_collection:str=None) -> SizeTimeChart:
        sizes: List[int] = []
        sizes_per_mode: Dict[str, List[int]] = dict()
        times: Dict[str, List[int]] = dict()

        for row in rows:
            if row.collection == collection and row.operation == operation:
                mode = row.mode
                if rename_collection is not None:
                    mode = rename_collection
                if mode not in sizes_per_mode:
                    sizes_per_mode[mode] = []
                sizes_per_mode[mode].append(row.size)
                if row.size not in sizes:
                    sizes.append(row.size)
                if mode not in times:
                    times[mode] = []
                times[mode].append(row.nanoseconds)

        for current in sizes_per_mode.values():
            assert current == sizes

        return SizeTimeChart(title, sizes, times)

    @staticmethod
    def from_rows(rows: List[Row]) -> 'Charts':
        return Charts(
            debug_creation=Charts.make_reduced_collections_chart(
                rows,
                'Creation of collections in debug (no optimizations) mode',
                'debug',
                'creation'),
            debug_find=Charts.make_all_collections_chart(
                rows,
                'Search for elements in collections in debug (no optimizations)'
                + ' mode',
                'debug',
                'find'),
            debug_inc_less_than=Charts.make_all_collections_chart(
                rows,
                'Increment all elements smaller than a certain value in'
                + ' collections in debug (no optimizations) mode',
                'debug',
                'find'),
            release_creation=Charts.make_reduced_collections_chart(
                rows,
                'Creation of collections in release (optimized) mode',
                'release',
                'creation'),
            release_find=Charts.make_all_collections_chart(
                rows,
                'Search for elements in collections in release (optimized)'
                + ' mode',
                'release',
                'find'),
            release_inc_less_than=Charts.make_all_collections_chart(
                rows,
                'Increment all elements smaller than a certain value in'
                + ' collections in release (optimized) mode',
                'debug',
                'find'),
            unsorted_array_creation=Charts.make_mode_chart(
                rows,
                'Creation of elements in unsorted array in debug and release'
                + ' (no optimizations vs optimized) modes',
                'good-local-array',
                'creation',
                rename_collection='unsorted-array'),
            sorted_array_creation=Charts.make_mode_chart(
                rows,
                'Creation of elements in sorted array in debug and release'
                + ' (no optimizations vs optimized) modes',
                'sorted-array',
                'creation'),
            linked_list_creation=Charts.make_mode_chart(
                rows,
                'Creation of elements in linked lists in debug and release'
                + ' (no optimizations vs optimized) modes',
                'linked-list-tree',
                'creation'),
            tree_creation=Charts.make_mode_chart(
                rows,
                'Creation of elements in trees in debug and release'
                + ' (no optimizations vs optimized) modes',
                'with-order-tree',
                'creation',
                rename_collection='tree'),
            good_local_array_find=Charts.make_mode_chart(
                rows,
                'Search for elements in unsorted, good-locality array in debug'
                + ' and release (no optimizations vs optimized) modes',
                'good-local-array',
                'find'),
            bad_local_array_find=Charts.make_mode_chart(
                rows,
                'Search for elements in unsorted, bad-locality array in debug'
                + ' and release (no optimizations vs optimized) modes',
                'bad-local-array',
                'find'),
            worse_local_array_find=Charts.make_mode_chart(
                rows,
                'Search for elements in unsorted, worse-locality array'
                + ' (worse than bad-local-array) in debug and release (no'
                + ' optimizations vs optimized) modes',
                'worse-local-array',
                'find'),
            with_order_tree_find=Charts.make_mode_chart(
                rows,
                'Search for elements in trees assuming correct order in debug'
                + ' and release (no optimizations vs optimized) modes',
                'with-order-tree',
                'find'),
            without_order_tree_find=Charts.make_mode_chart(
                rows,
                'Search for elements in trees not assuming correct order in'
                + ' debug and release (no optimizations vs optimized) modes',
                'without-order-tree',
                'find'),
            linked_list_find=Charts.make_mode_chart(
                rows,
                'Search for elements in linked list in debug'
                + ' and release (no optimizations vs optimized) modes',
                'linked-list',
                'find'),
            sorted_array_find=Charts.make_mode_chart(
                rows,
                'Search for elements in sorted array in debug'
                + ' and release (no optimizations vs optimized) modes',
                'linked-list',
                'find'),
            sorted_array_inc_less_than=Charts.make_mode_chart(
                rows,
                'Increments elements smaller than a  given number in sorted'
                + ' array in debug and release (no optimizations vs optimized)'
                + ' modes',
                'linked-list',
                'inc-less-than'),
            with_order_tree_inc_less_than=Charts.make_mode_chart(
                rows,
                'Increments elements smaller than a given number in trees'
                + '  assuming correct order in debug and release'
                + ' (no optimizations vs optimized) modes',
                'with-order-tree',
                'inc-less-than'),
            without_order_tree_inc_less_than=Charts.make_mode_chart(
                rows,
                'Increments elements smaller than a given number in trees not'
                + '  assuming correct order in debug and release'
                + ' (no optimizations vs optimized) modes',
                'without-order-tree',
                'inc-less-than'),
            linked_list_inc_less_than=Charts.make_mode_chart(
                rows,
                'Increments elements smaller than a  given number in linked'
                + ' list in debug and release'
                + ' (no optimizations vs optimized) modes',
                'linked-list',
                'inc-less-than'),
            good_local_array_inc_less_than=Charts.make_mode_chart(
                rows,
                'Increments elements smaller than a  given number in unsorted,'
                + ' good-locality array in debug and release'
                + ' (no optimizations vs optimized) modes',
                'good-local-array',
                'inc-less-than'),
            bad_local_array_inc_less_than=Charts.make_mode_chart(
                rows,
                'Increments elements smaller than a given number in unsorted,'
                + ' bad-locality array in debug'
                + 'and release (no optimizations vs optimized) modes',
                'bad-local-array',
                'inc-less-than'),
            worse_local_array_inc_less_than=Charts.make_mode_chart(
                rows,
                'Increments elements smaller than a given number in unsorted,'
                + ' worse-locality array (worse than bad-local-array) in debug'
                + 'and release (no optimizations vs optimized) modes',
                'worse-local-array',
                'inc-less-than'))

def main():
    if len(sys.argv) != 3:
        print('Usage: main.py [INPUT FILE] [OUTPUT DIR]', file=sys.stderr)
        sys.exit(1)
    rows = parse_rows(sys.argv[1])
    charts = Charts.from_rows(rows)
    charts.plot(sys.argv[2])

if __name__ == '__main__':
    main()
