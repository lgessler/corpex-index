import sys

def lines_stream(filename):
    with open(filename, 'r') as f:
        for line in f:
            yield line

def write_corrected_file(src, dst):
    with open(dst, 'w') as f:
        for line in lines_stream(src):
            vals = line.split('\t')
            if len(vals) != 3:
                vals[1] = vals[1][-1]
                vals.append('\n')
                f.write('\t'.join(vals))
            else:
                f.write(line)

if __name__ == '__main__':
    if len(sys.argv) != 3:
        print("Usage: \n\t python3 correct_tsv.py <src> <dst>")
        sys.exit(1)
    write_corrected_file(sys.argv[1], sys.argv[2])
