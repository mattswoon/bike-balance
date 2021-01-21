# bike-balance
Parse google timeline kml files to figure out how much further I've travelled by car than by bike.

Last year (2020), according to google timeline,
I spent about as much _time_ in a car than I did on my bike. This year I want
to try and match _distance_ instead. This is a little rust program to help me parse google 
kml files and pull out the relevant stats.

# Example

You can chuck all your daily kml files into a folder and set that folder as the environment variable `$BIKEBALANCE`, then simply call `bike-balance`. Alternatively you can pass this folder as the first argument to `bike-balance`.

atm it just prints the debug output of a polars dataframe giving the total amount driving and cycling, how much more you've driven than cycled, and how much you need to cycle each day/week to get that down to zero by the end of the year

```bash
>>> bike-balance
shape: (2, 2)
╭──────────┬──────────────╮
│ activity ┆ distance_sum │
│ ---      ┆ ---          │
│ str      ┆ f64          │
╞══════════╪══════════════╡
│ driving  ┆ 1.047042e6   │
├╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ cycling  ┆ 7.9537e4     │
╰──────────┴──────────────╯
Total debt is: 968km
You'll need to ride 2.80km per day or 19.63km per week to repay this debt
```
