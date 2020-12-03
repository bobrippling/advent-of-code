#!/usr/bin/perl
use strict;
use warnings;

my @grid;
while(<>){
	chomp;
	#s/ --->
	push @grid, $_;
}
my $h = @grid;
my $w = length $grid[0];

sub get {
	my($x, $y) = @_;

	$x -= $w while($x >= $w);

	return substr($grid[$y], $x, 1);
}

sub try_slope {
	my($xinc, $yinc) = @_;
	my $x = 0;
	my $y = 0;
	my $trees = 0;

	#sub check_and_inc {
	#	my $at = get($x, $y);
	#	if($at eq '#'){
	#		$trees++
	#	}else{
	#		if(!($at eq '.')){
	#			warn "invalid grid entry $at\n";
	#		}
	#	}
	#}

	while($y < $#grid){
		my $at = get($x, $y);
		if($at eq '#'){
			$trees++
		}else{
			if(!($at eq '.')){
				warn "invalid grid entry $at\n";
			}
		}

		#print $grid[$y];
		#print "$at\n";

		$x += $xinc;
		$y += $yinc;
	}
		my $at = get($x, $y);
		if($at eq '#'){
			$trees++
		}else{
			if(!($at eq '.')){
				warn "invalid grid entry $at\n";
			}
		}


	return $trees;
}

my $trees = try_slope(3, 1);
print "Part 1 (3,1): $trees\n";

my @slopes = (
	1, 1,
	3, 1,
	5, 1,
	7, 1,
	1, 2,
);
my $m = 1;
for(my $i = 0; $i < @slopes; $i += 2){
	my $x = $slopes[$i];
	my $y = $slopes[$i + 1];
	my $t = try_slope($x, $y);
	print "slope $x, $y: " . $t . "\n";
	$m *= $t;
}
print "$m\n";
