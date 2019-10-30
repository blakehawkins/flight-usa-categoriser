For tax purposes I need a recording of when I was in the united states. I use my.flightradar24.com to keep track of all my flights.

I wrote a quick program that categorises airports as "in" or "out" of the united states, and I pipe in data from flightradar to work out when I entered and left the USA.

```
$ xsv select From,To data.tsv | tail -n +2 | cargo run | xargs -I {} grep "{}" data.tsv
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/flight-usa-categoriser`
2019-09-21      UA901   N2331U  SFO     LHR     5,359   12:25   06:55   UAL     B77W
...
```
