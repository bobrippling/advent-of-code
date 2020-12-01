#!/usr/bin/perl

require '../lib.pl';

@n = contents("input.txt");

outer:
for(my $i = 0; $i < @n; $i++){
	for(my $j = 0; $j < @n; $j++){
		next if $i == $j;

		if($n[$i] + $n[$j] == 2020){
			# print "$n[$i] + $n[$j] == 2020, i=$i j=$j\n";
			printf "Part 1: %d\n", $n[$i] * $n[$j];
			last outer;
		}
	}
}

outer:
for(my $i = 0; $i < @n; $i++){
	for(my $j = 0; $j < @n; $j++){
		for(my $k = 0; $k < @n; $k++){
			next if $i == $j or $i == $k or $j == $k;

			if($n[$i] + $n[$j] + $n[$k] == 2020){
				#print "$n[$i] + $n[$j] + $n[$k] == 2020, i=$i j=$j k=$k\n";
				printf "Part 2: %d\n", $n[$i] * $n[$j] * $n[$k];
				last outer;
			}
		}
	}
}
