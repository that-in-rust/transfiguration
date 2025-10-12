# transfiguration

*turning software complexity into actually useful insights since tuesday*

## what is this

basically i got tired of reinventing the wheel every time i needed to understand how good software works, so i started collecting notes. turns out other people find this stuff useful too. who knew.

## how it works

everything's organized into `InspXXX_theme` folders because apparently i have opinions about naming conventions. each folder has `AXXX_document.md` files that you can actually read without falling asleep.

```
transfiguration/
├── Insp00_Tools/                 # scripts that actually work
├── Insp01_IDEAnalysis/           # why your ide is slow (and how to fix it)
├── Insp02_RustPerformance/       # making rust go brrrr  
├── Insp03_ParseltongueEvolution/ # how to turn a side project into actual value
├── Insp04_DownloadAnalysis/      # deconstructing how software gets to your computer
├── Insp05_EvolutionAnalysis/     # ide trends that matter (and ones that don't)
├── parseltongue/                 # the thing we're trying to make better
└── zed/                         # learning from people who actually know what they're doing
```

## what's inside

### tools that don't suck
`Insp00_Tools/` - utility scripts, batch downloaders, stuff that saves you from doing boring manual work

### ide archaeology  
`Insp01_IDEAnalysis/` - deep dives into how ides actually work when you lift the hood. includes some claude code analysis because why not.

### rust goes fast
`Insp02_RustPerformance/` - concrete ways to make rust code go from "meh" to "holy shit that's fast". includes that thing where 7mb of native code gives you 100x performance because computers are weird.

### business strategy without the bs
`Insp03_ParseltongueEvolution/` - how to take a parsing tool and turn it into something people actually want to pay for. turns out there's a method to this madness.

### software distribution deep dive
`Insp04_DownloadAnalysis/` - ever wonder how jetbrains gets their stuff onto your machine? or why some downloads just work and others are a nightmare? we deconstructed a bunch of installers to figure it out.

### ide evolution without the hype
`Insp05_EvolutionAnalysis/` - which ide trends actually matter and which ones are just marketing fluff. spoiler: most of them are marketing fluff.

## if you want to use this

for performance nerds:
```bash
cd Insp02_RustPerformance/
# start with the overview, then dive into the hybrid architecture stuff
```

for people who make product decisions:
```bash
cd Insp03_ParseltongueEvolution/
# actual business impact analysis, not just "wouldn't it be cool if..."
```

for ide builders:
```bash
cd Insp01_IDEAnalysis/
# learn from systems that actually work in production
```

## some numbers because apparently people like those

- 6μs graph operations (that's microseconds, not milliseconds)
- 1M+ symbols processed per second
- 24+ hours/year saved per developer (based on actual usage data)
- $2.4M/year roi for enterprise teams (assuming 1000 devs, which is conservative)

## ground rules

1. everything here is based on actual analysis, not speculation
2. if we reverse engineer something, it's legal black-box analysis only  
3. performance claims come with benchmarks
4. business impact comes with real numbers

## philosophy

most software analysis is either too academic to be useful or too shallow to be interesting. this tries to hit the sweet spot: deep enough to be valuable, practical enough to actually implement.

also, documentation that puts you to sleep is bad documentation.

---

**tl;dr**: detailed analysis of how good software works, organized so you can actually find stuff
