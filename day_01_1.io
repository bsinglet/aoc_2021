solution := Object clone
solution solve := method(
	fileHandle := File with("day01.txt");
	fileHandle openForReading;
	lines := fileHandle readLines;
	# convert the lines to integers
        for(index, 0, (lines size) - 1, lines atPut(index, (lines at(index)) asNumber));
        increases := 0;
	for(index, 1, (lines size) - 1, if(lines at(index) > lines at(index-1)) then (increases := increases + 1));
	"The number of increases is " print
	increases println);
solution solve
