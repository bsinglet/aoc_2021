import copy

def load_input(filename):
	my_lines = ''
	with open(filename, 'r') as my_file:
		my_lines = my_file.read()
	my_lines = my_lines.strip()
	my_lines = my_lines.split("\n")
	return my_lines

def parse_lines(my_lines, bit_count):
	for line in my_lines:
		for index in range(len(bit_count)):
			bit_count[index] += int(line[index])
	return bit_count

def find_rating(my_lines, bit_counts, num_lines, most_common=True):
	"""
	This is a very convoluted algorithm. We have to look at all possible
	numbers, pick the ones with the most common value for the first bit.
	Then, from that remaining pool, compare their second bit, eliminating
	the ones that don't have that most common value. And repeat until
	there can be only one.
	"""
	# we first consider all lines as proper candidates, and we're going to
	# steadily narrow this down until we finally have only 1 candidate.
	candidate_lines = set(list(range(len(my_lines))))
	for each_index in range(len(bit_counts)):
		remaining_lines = [my_lines[x] for x in candidate_lines]
		bit_counts = parse_lines(remaining_lines, [0 for x in range(len(bit_counts))])
		# when there's a tie between 0s and 1s, we bias toward 1 for
		# the oxygen rating and toward 0 for the co2 scrubber rating
		if bit_counts[each_index] == len(candidate_lines)/2:
			if most_common:
				goal_value = 1
			else:
				goal_value = 0
		else:
			goal_value = round(bit_counts[each_index]/len(candidate_lines))
			# we need to switch from 0 to 1 or vice versa if we're looking for the least common
			if not most_common:
				goal_value = 1 if goal_value == 0 else 0
		candidate_lines = {line_number for line_number in candidate_lines if int(my_lines[line_number][each_index]) == goal_value}
		# we're trying to narrow down to one number
		if len(candidate_lines) == 1:
			break
	if len(candidate_lines) != 1:
		print(f"Failure: we narrowed candidate_lines down to {len(candidate_lines)}.")
	filtered_line = my_lines[list(candidate_lines)[0]]
	print(f"The remaining candidate is {filtered_line}")
	my_rating = int("".join(str(x) for x in filtered_line), 2)
	return my_rating

def main():
	my_lines = load_input('day03.txt')
	num_lines = len(my_lines)
	num_bits = len(my_lines[0])
	bit_count = [0 for x in range(num_bits)]
	bit_count = parse_lines(my_lines, bit_count)
	gamma_rate = [round(x/num_lines) for x in bit_count]
	epsilon_rate = list()
	for bit in gamma_rate:
		epsilon_rate.append(0 if bit == 1 else 1)
	gamma_rate = int("".join(str(x) for x in gamma_rate), 2)
	epsilon_rate = int("".join(str(x) for x in epsilon_rate), 2)
	print(f"The gamma rate is {gamma_rate}, the episolon rate is {epsilon_rate}.")
	print(f"The power consumption of the submarine (the product of these two rates) is {gamma_rate * epsilon_rate}.")
	oxygen_rating = find_rating(my_lines=my_lines, bit_counts=bit_count, num_lines=num_lines, most_common=True)
	co2_rating = find_rating(my_lines=my_lines, bit_counts=bit_count, num_lines=num_lines, most_common=False)
	print(f"The oxygen rating is {oxygen_rating}, and the co2 rating is {co2_rating}.")
	print(f"Therefore, the life support rating (the product of these two rates) is {oxygen_rating * co2_rating}.")


if __name__ == '__main__':
	main()
