#!/usr/bin/perl
use warnings;
use strict;

# byr (Birth Year) - four digits; at least 1920 and at most 2002.
sub valid_byr {
	my $byr = shift();
	return (length($byr) == 4 and (1920 <= $byr and $byr <= 2002));
}

# iyr (Issue Year) - four digits; at least 2010 and at most 2020.
sub valid_iyr {
	my $iyr = shift();
	return (length($iyr) == 4 and (2010 <= $iyr and $iyr <= 2020));
}

# eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
sub valid_eyr {
	my $eyr = shift();
	return (length($eyr) == 4 and (2020 <= $eyr and $eyr <= 2030));
}

# hgt (Height) - a number followed by either cm or in:
#     If cm, the number must be at least 150 and at most 193.
#     If in, the number must be at least 59 and at most 76.
sub valid_hgt {
	my $hgt = shift();
	if($hgt =~ /^(\d+)(cm|in)$/){
		my($n, $typ) = ($1, $2);
		if($typ eq "cm"){
			return(150 <= $n and $n <= 193);
		}
		return(59 <= $n and $n <= 76);
	}
	return 0;
}

# hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
sub valid_hcl {
	my $hcl = shift();
	return scalar($hcl =~ /^#[0-9a-f]{6}$/)
}

# ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
sub valid_ecl {
	my $ecl = shift();
	return scalar($ecl =~ /^(amb|blu|brn|gry|grn|hzl|oth)$/);
}

# pid (Passport ID) - a nine-digit number, including leading zeroes.
sub valid_pid {
	my $pid = shift();
	return scalar($pid =~ /^\d{9}$/);
}

# cid (Country ID) - ignored, missing or not.

my @req = (
  "byr", # (Birth Year)
  "iyr", # (Issue Year)
  "eyr", # (Expiration Year)
  "hgt", # (Height)
  "hcl", # (Hair Color)
  "ecl", # (Eye Color)
  "pid", # (Passport ID)
  #"cid", # (Country ID) [permitted missing]
);

my %validation = (
	"byr" => \&valid_byr,
	"iyr" => \&valid_iyr,
	"eyr" => \&valid_eyr,
	"hgt" => \&valid_hgt,
	"hcl" => \&valid_hcl,
	"ecl" => \&valid_ecl,
	"pid" => \&valid_pid,
);

sub lines_to_passport {
	my @ents;
	for(@_){
		push @ents, split / /;
	}

	my $passport = {};
	for(@ents){
		die "couldn't match $_" unless /(.*):(.*)/;
		$passport->{$1} = $2;
	}
	return $passport;
}

my @lines;
my @passports;

while(<>){
	chomp;
	if(!length){
		push @passports, lines_to_passport(@lines);
		@lines = ();
	}else{
		push @lines, $_;
	}
}
if(@lines){
	push @passports, lines_to_passport(@lines);
}

my $valids = 0;
for(@passports){
	my %pp = %{$_};
	my $valid = 1;
	for(@req){
		if(!$pp{$_}){
			$valid = 0;
			last;
		}
	}
	if($valid){
		$valids++;
	}
}
print "Part 1: $valids\n";

$valids = 0;
for(@passports){
	my %pp = %{$_};
	my $valid = 1;
	for my $name (@req){
		my $fn = $validation{$name};
		my $v = $pp{$name};
		if(!$v or !$fn->($v)){
			$valid = 0;
			last;
		}
	}
	if($valid){
		$valids++;
	}
}
print "Part 2: $valids\n";
