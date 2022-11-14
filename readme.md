## csv_stacker

A simple binary utility to vertically stack a bunch of CSV files with partially matching column headers.

Can be configured using Config.toml to define 3 parameters:
1. csv_dir_path: the path to the directory where the csvs you want to stack are held.
2. outfile: the filename to write the stacked csv out to.
3. colnames: a list of column headers you want to select for in the stacked csv file.

I use this to aggregate individual stock trading transaction logs into a single combined csv history file.
