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


cache = {"dac": {}, "fft": {}, "both": {}, "none": {}}
cache_paths = {}
sol_cnt = 0

def solve_part_2(lines):
    hs = parse_input(lines)
    count_paths(hs, "svr", set())
    return sol_cnt


def count_paths(hs, current, path):
    my_cnt = 0
    for nb in hs[current]:
        if nb not in path:
            if nb == "out":
                if "fft" in path and "dac" in path:
                    my_cnt += 1
            elif nb in cache:
                my_cnt += cache[nb]
            else:
                new_path_set = path.copy()
                new_path_set.add(nb)
                my_cnt += count_paths(hs, nb, new_path_set)
    cache[current] = my_cnt 
    if "dac" in path and "fft" in path:
        global sol_cnt
        sol_cnt += my_cnt                         
    return my_cnt
                
print(solve_part_2(read_lines("input_test_2")))