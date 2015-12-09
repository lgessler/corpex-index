"""
Takes in the HindMonoCorp plaintext file, sorts it, and splits it into N equal
parts.
"""

import sys
from math import ceil

def sort_file(filename):
    """
    Read in a list (separated by newline chars), sort in place, and return.
    """

    lines = []
    with open(filename, 'r') as f:
        for line in f:
            lines.append(line)

    lines.sort()
    return lines

if __name__ == '__main__':
    try:
        filename = str(sys.argv[1])
        num_shards = int(sys.argv[2])
        dst_dir = str(sys.argv[3])
        assert len(sys.argv) == 4
        assert dst_dir[-1] == '/'
    except:
        print("""
    Usage:
        
        python3 shard.py <filename> <num shards> <destination directory>

    e.g., 

        python3 shard.py HindMonoCorp05.plaintext 8 ./shards/
        """)
        sys.exit(1)

    try:
        sorted_lines = sort_file(filename)
    except MemoryError:
        print("Ran out of memory while trying to sort the file.")
        sys.exit(1)
    
    num_lines = len(sorted_lines)
    shard_size = ceil(num_lines / num_shards)
    print(shard_size)

    for i in range(num_shards):
        with open("{}{}.shard{}".format(dst_dir, filename, i), 'w') as f:
            for j in range(i*shard_size, min((i+1)*shard_size, len(sorted_lines))):
                f.write(sorted_lines[j])
    





