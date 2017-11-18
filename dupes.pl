#!/usr/bin/perl

# Create a map for the names
my %names;

# Parse names from file
if (open (F, "$ARGV[0]")) {
    while (read(F, $buf, 4096) > 0) {
        @matches = $buf =~ m/<name>(\w+)/g;
        foreach my $el (@matches) {
            if (!exists $names{$el}) {
                $names{$el} = 1;
            } else {
                $names{$el}++;
            }
        }
    }
}

# Print the duplicates
foreach my $el (keys %names) {
    if ($names{$el} > 1) {
        print("$el: $names{$el}\n");
    }
}
