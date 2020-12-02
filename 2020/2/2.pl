#!/usr/bin/perl

sub valid_pw {
	my($min, $max, $ch, $pw) = @_;
	my $n = (my $tmp = $pw) =~ s/$ch//g;

	return $min <= $n && $n <= $max;
}

require '../lib.pl';

my @lines = contents("input.txt");

my $valid = 0;
for(@lines){
	die unless /(\d+)-(\d+) (.): (.+)/;
	my($min, $max, $ch, $pw) = ($1, $2, $3, $4);

	if(valid_pw($min, $max, $ch, $pw)){
		$valid++;
	}
}
print "Part 1: $valid\n";

sub valid_pw2 {
	my($pos1, $pos2, $ch, $pw) = @_;
	my $c1 = substr($pw, $pos1, 1) eq $ch;
	my $c2 = substr($pw, $pos2, 1) eq $ch;

	return $c1 ^ $c2;
}

$valid = 0;
for(@lines){
	die unless /(\d+)-(\d+) (.): (.+)/;
	my($pos1, $pos2, $ch, $pw) = ($1, $2, $3, $4);
	$pos1--;
	$pos2--;

	if(valid_pw2($pos1, $pos2, $ch, $pw)){
		$valid++;
	}
}

print "Part 2: $valid\n";
