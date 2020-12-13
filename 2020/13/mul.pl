chomp(my $earliest = <>);

chomp(my $times = <>);
@times = grep /\d+/, split /,/, $times;
#print "times:\n";
#print "$_\n" for @times;
my @mut_times = @times;

my $n = 2;

outer:
while(1){
	my $found = 0;
	my $min_time = 999999999999;

	for(my $i = 0; $i < @times; $i++){
		my $time = $mut_times[$i];
		if($time >= $earliest){
			my $wait_maybe = $time - $earliest;
			if($wait_maybe < $min_time){
				print "found, wait_maybe=$wait_maybe\n";
				$wait = $wait_maybe;
				$bus_id = $times[$i];
				$min_time = $wait;
				$found = 1;
			}
		}
	}

	if($found){
		printf "%d (bus_id=$bus_id, wait=$wait, earliest=$earliest)\n", $bus_id * $wait;
		last;
	}

	#print "trying $n...\n";
	for(my $i = 0; $i < @times; $i++){
		$mut_times[$i] = $times[$i] * $n;
	}
	$n++;
}
