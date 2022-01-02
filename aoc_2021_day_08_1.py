
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
	potential_candidates = [x for x in range(min(my_lines), max(my_lines)+1)]
	best_fuel = None
	for location in potential_candidates:
		# each step a crab submarine takes costs 1 more unit of fuel than the last
		# 1 step costs 1, 2 steps costs 3 = 1 + 2, 3 steps costs 6 = 1 + 2 + 3, etc
		# in other words, the cost of moving n spaces is equal to the
		# nth triangular number = (n*(n+1))/2
		total_fuel = sum([(abs(location-x)*(abs(location-x)+1))/2 for x in my_lines])
		if best_fuel is None:
			best_fuel = total_fuel
		elif total_fuel < best_fuel:
			best_fuel = total_fuel
	return int(best_fuel)

def main():
	my_lines = load_input('day07.txt')
	required_fuel = minimize_fuel(my_lines)
	print(f"The crab submarines need to expend {required_fuel} units of fuel to align.")


if __name__ == '__main__':
	main()

