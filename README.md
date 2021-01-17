# bike-balance
Parse google timeline kml files to figure out how much further I've travelled by car than by bike.

Last year (2020), according to google timeline,
I spent about as much _time_ in a car than I did on my bike. This year I want
to try and match _distance_ instead. This is a little rust program to help me parse google 
kml files and pull out the relevant stats.

# Example

ATM just prints the debug output of a polars dataframe. One day I'll make it print
something pretty that can be catted straight to a tweet.

```bash
>>> bike-balance ./data/
shape: (2, 2)
╭──────────┬──────────────╮
│ activity ┆ distance_sum │
│ ---      ┆ ---          │
│ str      ┆ f64          │
╞══════════╪══════════════╡
│ driving  ┆ 1.033885e6   │
├╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ cycling  ┆ 4.2985e4     │
╰──────────┴──────────────╯
Total debt is: 991km
```
