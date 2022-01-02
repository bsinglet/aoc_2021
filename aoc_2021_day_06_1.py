import re
import copy
from collections import defaultdict

def load_input(filename):
	my_lines = ''
	with open(filename, 'r') as my_file:
		my_lines = my_file.read()
	my_lines = my_lines.strip()
	# change any repeated spaces to single spaces
	my_lines = my_lines.replace('  ', ' ')
	my_lines = [int(x.strip()) for x in my_lines.split(',')]
	return my_lines

def simulate_lanternfish(my_lines, days_to_simulate):
	population = {x: 0 for x in range(9)}
	for index in population.keys():
		population[index] = my_lines.count(index)
	for day_number in range(days_to_simulate):
		next_population = {x: 0 for x in range(9)}
		next_population[8] = population[0]
		next_population[7] = population[8]
		next_population[6] = population[0] + population[7]
		for index in range(6):
			next_population[index] = population[index+1]
		population = {x: next_population[x] for x in next_population.keys()}
	return sum(population.values())

def main():
	my_lines = load_input('day06.txt')
	population = simulate_lanternfish(my_lines, 80)
	print(f"After 80 days, the final population is {population}.")


if __name__ == '__main__':
	main()

