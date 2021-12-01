#!/usr/bin/perl

use strict;
use warnings;

my @input = <>;

sub part1 {
	my $last = 999999;
	my $incs = 0;
	for(@input){
		chomp(my $n = $_);
		if($n > $last){
			$incs++;
		}
		$last = $n;
	}
	print "Part 1: $incs\n";
}

sub part2 {
	my $last = 99999999;
	my $incs = 0;

	for(my $i = 0; $i < @input; $i++){
		if($i+2 >= @input){
			last;
		}
		my @window = ($input[$i], $input[$i+1], $input[$i+2]);
		my $winsum = $window[0]+$window[1]+$window[2];

		if($winsum > $last){
			$incs++;
		}
		$last = $winsum;
	}
	print "Part 2: $incs\n";
}

part1;
part2;
