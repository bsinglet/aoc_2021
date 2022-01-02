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
	my_lines = [x.strip() for x in my_lines.split('\n')]
	return my_lines

def parse_lines(my_lines):
	matching_regex = '(\d+),(\d+)\s+->\s+(\d+),(\d+)'
	result = list()
	for line in my_lines:
		matches = re.search(matching_regex, line)
		if matches is None:
			# print(f"No matches for {line}")
			continue
		matches = [int(matches.group(x)) for x in range(1,5)]
		result.append(((matches[0], matches[1]), (matches[2], matches[3])))
	return result

def update_extremes(extremes, line_segment):
	min_x, min_y = extremes[0]
	max_x, max_y = extremes[1]
	for x in [line_segment[0][0], line_segment[1][0]]:
		if x < min_x:
			min_x = x
		if x > max_x:
			max_x = x
	for y in [line_segment[0][1], line_segment[1][1]]:
                if y < min_y:
                        min_y = y
                if y > max_y:
                        max_y = y
	return ((min_x, min_y), (max_x, max_y))

def plot_line_segments(parsed_lines):
	# line_segments = list()
	# extremes = ((0, 0), (0, 0))
	sea_floor = defaultdict(int)
	for each_segment in parsed_lines:
		# extremes = update_extremes(extremes, each_segment)
		(x1, y1) = each_segment[0]
		(x2, y2) = each_segment[1]
		# print(f"Processing segment {each_segment}")
		if x1 == x2 and y1 != y2:
			# vertical line
			# swap the y's if they're in descending order
			if y1 > y2:
				y1, y2 = y2, y1
			for y in range(y1, y2 + 1):
				sea_floor[(x1, y)] += 1
		elif x1 != x2 and y1 == y2:
			# horizontal line
			# swap the y's if they're in descending order
			if x1 > x2:
				x1, x2 = x2, x1
			for x in range(x1, x2 + 1):
				sea_floor[(x, y1)] += 1
		else:
			# this is fine, we ignore lines that aren't vertical or horizontal
			continue
	return sea_floor


def main():
	my_lines = load_input('day05.txt')
	parsed_lines = parse_lines(my_lines)
	sea_floor = plot_line_segments(parsed_lines)
	overlap_points = len([x for x in sea_floor.values() if x > 1])
	print(f"Found {overlap_points} points where 2 or more lines overlap.")


if __name__ == '__main__':
	main()

