def load_input(filename):
	my_lines = ''
	with open(filename, 'r') as my_file:
		my_lines = my_file.read()
	my_lines = my_lines.strip()
	my_lines = my_lines.split("\n")
	return my_lines

def move(move_pair, x, y):
	direction, distance = move_pair.split(" ")
	direction = direction.strip()
	distance = int(distance.strip())
	if direction == 'forward':
		x += distance
	elif direction == 'down':
		y += distance
	elif direction == 'up':
		y -= distance
	else:
		print(f"Unrecognized direction {direction} with distance {distance}.")
		return None
	return x, y

def main():
	my_lines = load_input('day02.txt')
	x, y = (0, 0)
	for each_line in my_lines:
		x, y = move(each_line, x, y)
	print(f"The final position is ({x}, {y}).")
	print(f"And the product of these coordinates is {x * y}")


if __name__ == '__main__':
	main()
