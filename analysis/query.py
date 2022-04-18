import argparse
import sys
from row import Row, parse_rows, format_size, format_time
from typing import Dict, NamedTuple, List, Generator, Tuple, Optional, Any

class Key(NamedTuple):
    mode: str
    size: str 
    operation: str
    collection: str

def make_database(rows: List[Row]) -> Dict[Key, str]:
    db = dict()
    for row in rows:
        db[Key(
            mode=row.mode,
            size=format_size(row.size),
            operation=row.operation,
            collection=row.collection)] = format_time(row.nanoseconds)
    return db

def main():
    parser = argparse.ArgumentParser(description='query time of a record')
    parser.add_argument(
            '-f, --file',
            metavar='F',
            type=str,
            dest='file',
            required=True)
    parser.add_argument(
            '-m, --mode',
            metavar='M',
            type=str,
            dest='mode',
            required=True)
    parser.add_argument(
            '-s, --size',
            metavar='S',
            type=str,
            dest='size',
            required=True)
    parser.add_argument(
            '-o, --operation',
            metavar='O',
            type=str,
            dest='operation',
            required=True)
    parser.add_argument(
            '-c, --collection',
            metavar='C',
            type=str,
            dest='collection',
            required=True)
    args = parser.parse_args()

    key = Key(
            mode=args.mode,
            size=args.size,
            operation=args.operation,
            collection=args.collection)

    rows = parse_rows(args.file)
    db = make_database(rows)
    if key in db:
        print(db[key])
    else:
        print('Not found', file=sys.stderr)
        exit(1)

if __name__ == '__main__':
    main()
