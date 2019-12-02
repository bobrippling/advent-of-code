sub contents {
	my $f = shift();
	open(my $fh, '<', $f) or die "open $f: $!\n";
	my @contents = <$fh>;
	close($fh);
	return @contents;
}

1
