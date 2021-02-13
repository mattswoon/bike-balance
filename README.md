# bike-balance
Parse google timeline kml files to figure out how much further I've travelled by car than by bike.

Last year (2020), according to google timeline,
I spent about as much _time_ in a car than I did on my bike. This year I want
to try and match _distance_ instead. This is a little rust program to help me parse google 
kml files and pull out the relevant stats.

# Example

You can chuck all your daily kml files into a folder and set that folder as the environment variable `$BIKEBALANCE`, then simply call `bike-balance`. Alternatively you can pass this folder as the first argument to `bike-balance`.

atm it prints 
 - how much more you've driven than cycled, 
 - how much you need to cycle each day/week to get that down to zero by the end of the year
 - your driving/cycling totals from the last week
 - your driving/cycling totals

```bash
>>> bike-balance
Total debt is: 945km
To repay this debt you'll need to ride:
	2.94km per day or 
	20.55km per week

Over the last week you've:
	driven 8.77km
	cycled 50.26km

In total you've
	driven 1208.85km
	cycled 263.76km
```
