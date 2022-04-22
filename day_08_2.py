
def load_input(filename):
	my_lines = ''
	with open(filename, 'r') as my_file:
		my_lines = my_file.read()
	my_lines = my_lines.strip()
	# change any repeated spaces to single spaces
	my_lines = my_lines.replace('  ', ' ')
	my_lines = [x.strip() for x in my_lines.split('\n')]
	return my_lines

def process_lines(lines: list) -> int:
    segments_to_digits = {
        2: 1,
        3: 7,
        4: 4,
        7: 8
    }
    output_values = list()
    for each_line in lines:
        inputs, outputs = [x.strip() for x in each_line.split('|')]
        inputs = [x.strip() for x in inputs.split(' ')]
        outputs = [x.strip() for x in outputs.split(' ')]

        # this question is very confusingly written. I ended up having to
        # use the solution from this site, modifying it to suit my own style:
        # https://medium.com/interviewnoodle/seven-segment-search-day-8-advent-of-code-2021-21e5bc965005

        temporary_mapping = dict()
        # find all of our known words
        for input_word in inputs:
            if len(input_word) in segments_to_digits.keys():
                temporary_mapping[segments_to_digits[len(input_word)]] = ''.join(sorted(input_word))

        # Find the digit 6
        for input_word in inputs:
            # the digit 1 has a single segment not found in 6
            if len(input_word) == 6 and any(character not in input_word for character in temporary_mapping[1]):
                temporary_mapping[6] = ''.join(sorted(input_word))
                break

        # Find the digit 0
        for input_word in inputs:
            # the digit 4 has a single segment not found in 0
            if len(input_word) == 6 and ''.join(sorted(input_word)) not in temporary_mapping.values() and any(character not in input_word for character in temporary_mapping[4]):
                temporary_mapping[0] = ''.join(sorted(input_word))
                break

        # Find the digit 9
        for input_word in inputs:
            # the digit 9 is the only remaining digit of length 6
            if len(input_word) == 6 and ''.join(sorted(input_word)) not in temporary_mapping.values():
                temporary_mapping[9] = ''.join(sorted(input_word))
                break

        # Find the digit 5
        for input_word in inputs:
            # all of the segments in the digit 5 are also in digit 6
            if len(input_word) == 5 and all(character in temporary_mapping[6] for character in input_word):
                temporary_mapping[5] = ''.join(sorted(input_word))
                break

        # Find the digit 3
        for input_word in inputs:
            # all of the segments in the digit 3 are also in digit 9
            if len(input_word) == 5 and ''.join(sorted(input_word)) not in temporary_mapping.values() and all(character in temporary_mapping[9] for character in input_word):
                temporary_mapping[3] = ''.join(sorted(input_word))
                break

        # Find the digit 2
        for input_word in inputs:
            # the digit 2 is the only remaining digit of length 5
            if len(input_word) == 5 and ''.join(sorted(input_word)) not in temporary_mapping.values():
                temporary_mapping[2] = ''.join(sorted(input_word))
                break

        # convert the output digit based on the mappings we've found for this line
        output_digits = list()
        for output_word in outputs:
            for each_key in temporary_mapping.keys():
                if temporary_mapping[each_key] == ''.join(sorted(output_word)):
                    output_digits.append(each_key)
                    break

        if len(output_digits) != 4:
            print(f"ERROR: Failed to find all the output digits for line:\n {each_line}")
            return None

        # store the four-digit output in a list that we'll sum together at
        # the end.
        output_digits = ''.join([str(x) for x in output_digits])
        output_digits = int(output_digits)
        output_values.append(output_digits)
    print(output_values)
    return sum(output_values)


def main():
    lines = load_input('day08_input.txt')
    print(f"The sum of all the four-digit output values is {str(process_lines(lines))}.")


if __name__ == '__main__':
    main()
