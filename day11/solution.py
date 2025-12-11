def read_lines(filename):
    with open(filename, 'r') as f:
        return [line.strip() for line in f.readlines()]

def parse_input(lines):
    result = {}
    for line in lines:
        if ": " in line:
            start, targets = line.split(": ")
            target_list = targets.split()
            result[start] = target_list
    return result


cache = {} # Stores tuple (none, dac, fft, both)

def solve_part_2(lines):
    hs = parse_input(lines)
    _, _, _, both_cnt = count_paths(hs, "svr", set())
    return both_cnt


def count_paths(hs, current, path):
    if current == "out":
        return 1, 0, 0, 0
    
    if current in cache:
        return cache[current]
    
    # Four counters for paths starting from here
    c_none, c_dac, c_fft, c_both = 0, 0, 0, 0
    
    if current in hs:
        for nb in hs[current]:
            if nb not in path:
                r_none, r_dac, r_fft, r_both = count_paths(hs, nb, path | {current})
                c_none += r_none
                c_dac += r_dac
                c_fft += r_fft
                c_both += r_both

    if current == "dac":
        res = (0, c_none + c_dac, 0, c_fft + c_both)
    elif current == "fft":
        res = (0, 0, c_none + c_fft, c_dac + c_both)
    else:
        res = (c_none, c_dac, c_fft, c_both)
        
    cache[current] = res
    return res
                
print(solve_part_2(read_lines("input_1")))