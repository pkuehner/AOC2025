import re


def parse_line(line):
    # Parse []
    bracket_match = re.search(r"\[(.*?)\]", line)
    if bracket_match:
        bracket_content = bracket_match.group(1)
        array_part = [1 if c == "#" else 0 for c in bracket_content]
    else:
        array_part = []

    # Parse ()
    paren_matches = re.findall(r"\((.*?)\)", line)
    paren_lists = []
    for match in paren_matches:
        if match.strip():
            nums = [int(x.strip()) for x in match.split(",")]
            paren_lists.append(nums)
        else:
            paren_lists.append([])

    # Parse {}
    brace_match = re.search(r"\{(.*?)\}", line)
    if brace_match:
        brace_content = brace_match.group(1)
        if brace_content.strip():
            brace_list = [int(x.strip()) for x in brace_content.split(",")]
        else:
            brace_list = []
    else:
        brace_list = []

    return array_part, paren_lists, brace_list


def main():
    try:
        with open("input_1", "r") as f:
            sol_1 = 0
            sol_2 = 0

            for line in f:
                line = line.strip()
                if not line:
                    continue

                array_part, paren_lists, brace_list = parse_line(line)

                sol_1 += solve_part_1([0] * len(array_part), array_part, paren_lists)
                sol_2 += solve_part_2([0] * len(brace_list), brace_list, paren_lists)
                print(line, sol_2)
            print(sol_1)
            print(sol_2)
    except FileNotFoundError:
        print("Error: input_test file not found.")


def solve_part_1(current_lights, lights_correct, buttons):
    queue = [(current_lights, 0)]
    seen = set()

    while len(queue) > 0:
        current_lights, steps = queue.pop(0)
        if tuple(current_lights) in seen:
            continue
        else:
            seen.add(tuple(current_lights))

        if current_lights == lights_correct:
            return steps
        else:
            for button in buttons:
                queue.append(
                    (press_light_button(current_lights.copy(), button), steps + 1)
                )


def solve_part_2(current_counters, counters_expected, buttons):
    from scipy.optimize import linprog
    import numpy as np

    num_counters = len(counters_expected)
    num_buttons = len(buttons)
    
    # A matrix: rows are counters, cols are buttons
    A = np.zeros((num_counters, num_buttons))
    for j, btn in enumerate(buttons):
        for counter_idx in btn:
            if counter_idx < num_counters:
                A[counter_idx, j] = 1
                
    # b vector: expected - current
    b = np.array(counters_expected) - np.array(current_counters)
    
    # Objective: minimize sum(x)
    c = np.ones(num_buttons)
    
    # Bounds: x >= 0
    bounds = [(0, None) for _ in range(num_buttons)]
    
    # Integrality: 1 for integer variables
    integrality = np.ones(num_buttons)

    # Solve with integrality constraints (MILP)
    # Note: 'highs' method is required for integrality
    res = linprog(c, A_eq=A, b_eq=b, bounds=bounds, method='highs', integrality=integrality)
    
    if res.success:
        return int(np.round(res.fun))
    
    return 0


def press_light_button(lights, button):
    for idx in button:
        if 0 <= idx < len(lights):
            lights[idx] ^= 1  # Toggle the light
    return lights


def press_joltage_button(counters, button):
    for idx in button:
        if 0 <= idx < len(counters):
            counters[idx] += 1  # Toggle the light
    return counters


if __name__ == "__main__":
    main()
