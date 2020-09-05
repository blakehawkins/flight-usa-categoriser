For tax purposes I need a recording of when I was in the united states. I use my.flightradar24.com to keep track of all my flights.

I wrote a quick program that categorises airports as "in" or "out" of the united states, and I pipe in data from flightradar to work out when I entered and left the USA.

```
$ xsv sort data.csv | xsv select From,To | tail -n +2 | cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/flight-usa-categoriser`
JFK,HEL
HEL,JFK
...
```
