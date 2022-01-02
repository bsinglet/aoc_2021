import pandas as pd

def load_input(filename):
	my_lines = ''
	with open(filename, 'r') as my_file:
		my_lines = my_file.read()
	my_lines = my_lines.strip()
	# change any repeated spaces to single spaces
	my_lines = my_lines.replace('  ', ' ')
	my_lines = [int(x.strip()) for x in my_lines.split(',')]
	return my_lines

def minimize_fuel(my_lines):
	df = pd.DataFrame(my_lines)
	median = df.median()
	total_fuel = sum([abs(median - x) for x in df[0]])
	return int(total_fuel)

def main():
	my_lines = load_input('day07.txt')
	required_fuel = minimize_fuel(my_lines)
	print(f"The crab submarines need to expend {required_fuel} units of fuel to align.")


if __name__ == '__main__':
	main()

