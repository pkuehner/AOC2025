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
                print("Done")
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
    queue = [(current_counters, 0)]
    seen = set()

    while len(queue) > 0:
        current_counters, steps = queue.pop(0)
        print(current_counters, steps)
        for i in range(len(current_counters)):
            if current_counters[i] > counters_expected[i]:
                seen.add(tuple(current_counters))
        print(current_counters)
        if tuple(current_counters) in seen:
            continue
        else:
            seen.add(tuple(current_counters))

        if current_counters == counters_expected:
            return steps
        else:
            for button in buttons:
                result = press_joltage_button(current_counters.copy(), button)
                for i in range(len(result)):
                    if result[i] > counters_expected[i]:
                        seen.add(tuple(result))
                if(tuple(result) not in seen):
                    queue.append((result, steps + 1))


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
