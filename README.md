

## pnode_info_rs (Prometheus Node Info written in rust-lang)

Simple scaffolding project to build your own [prometheus](https://prometheus.io/) integration on Linux.


## Why create yet another monitoring / exporter?

  1. [Node-Exporter](https://github.com/prometheus/node_exporter) - I bump into a similar [issue](https://github.com/prometheus/node_exporter/issues/1880) 
  2. Was coding a hobby project written on rust-lang already on target linux rpi4 arch, why not write this like a hackaton?
  3. Learning prometheus exporter is a bit of a challenge, no GOOD simple examples at the internet.
 
## What is not?

This is a scaffold project so if you plan to use this, I assume you know what you are doing.

However if you just want to use an exporter / agent, I suggest you folks use [Node-Exporter](https://github.com/prometheus/node_exporter) as that is a really great project and can be configurable, built by a lot of really smart people and you folks can contribute and help out that project. 

Why use this instead? Maybe a hobby or learning?, or either you are crazy or crazier, if you are doing this to yourself you accept the risk right? As you'll be maintaining the code, personally message library people (and most of the time they are a grumpy bunch), so you need to be civil and cool headed first, then explain thoroughly what was the issue, propose patches and or just pointing out where the compiler just blow up to lessen their burden on searching things, because that will trigger a regression test and it will take forever (and that will add more to the grumpy part so just imagine yourself on their shoes) and that responsibility is like taking care of a child and you know what trouble it brings.

If you are a corp. then I applaud you as that will open up rust-lang dev's work-market-availability.


## Note DO NOT be a snowflake
If I like your PR then it can be merged., If not there is this thing called a "FORK". This project is called "SCAFFOLD" you copy and build something on top of it, then remove other things that is worthless.., optimize it the way you like it.., you get the point.



## License
0BSD.







