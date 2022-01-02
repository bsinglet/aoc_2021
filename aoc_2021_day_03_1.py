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


if __name__ == '__main__':
	main()
