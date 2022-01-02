solution := Object clone
solution solve := method(
	fileHandle := File with("day01.txt");
	fileHandle openForReading;
	lines := fileHandle readLines;
	# convert the lines to integers
        for(index, 0, (lines size) - 1, lines atPut(index, (lines at(index)) asNumber));
        increases := 0;
	previous_sum := (lines at(0)) + (lines at(1)) + (lines at(2));
	for(index, 3, (lines size) - 1, 
		current_sum := (lines at(index)) + (lines at(index-1)) + (lines at(index-2));
		if (current_sum > previous_sum) then (increases := increases + 1);
		previous_sum := current_sum);
	"The number of increases is " print
	increases println);
solution solve
