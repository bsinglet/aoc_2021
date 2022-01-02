def load_input(filename):
	my_lines = ''
	with open(filename, 'r') as my_file:
		my_lines = my_file.read()
	my_lines = my_lines.strip()
	my_lines = my_lines.split("\n")
	return my_lines

def move(move_pair, aim, x, y):
	direction, distance = move_pair.split(" ")
	direction = direction.strip()
	distance = int(distance.strip())
	if direction == 'forward':
		x += distance
		y += aim * distance
	elif direction == 'down':
		aim += distance
	elif direction == 'up':
		aim -= distance
	else:
		print(f"Unrecognized direction {direction} with distance {distance}.")
		return None
	return aim, x, y

def main():
	my_lines = load_input('day02.txt')
	aim, x, y = (0, 0, 0)
	for each_line in my_lines:
		aim, x, y = move(move_pair=each_line, aim=aim, x=x, y=y)
	print(f"The final aim is {aim} and final position is ({x}, {y}).")
	print(f"And the product of these coordinates is {x * y}")


if __name__ == '__main__':
	main()
