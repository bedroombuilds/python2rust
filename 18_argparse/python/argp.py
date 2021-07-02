import argparse
from enum import Enum


class Categories(Enum):
    Science = 28
    People = 22
    Comedy = 23


VALID_CATEGORIES = [c.name for c in Categories]

if __name__ == '__main__':
    parser = argparse.ArgumentParser(formatter_class=argparse.ArgumentDefaultsHelpFormatter)
    parser.add_argument('--file', required=True, help='Video file to upload')
    parser.add_argument('--title', help='Video title')
    parser.add_argument('--category',
                        choices=VALID_CATEGORIES,
                        default='Science',
                        help='video category.')
    parser.add_argument('--verbose', action='store_true', help='show more output')
    parser.add_argument('-n',
                        '--name',
                        dest='names',
                        action='append',
                        help="provides names to greet")
    args = parser.parse_args()

    args.category = Categories.__getitem__(args.category)
    print(args)
