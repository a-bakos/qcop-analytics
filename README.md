# QCop Intel - Search Insights

**Status: WORK IN PROGRESS!**

QCop Intel is a Rust program designed to analyze and provide insights into user search behavior on a WordPress website.
It works in conjunction with a WordPress plugin I wrote, called QueryCop or QCop, that collects search-related data and
stores it in a dedicated database table.

This program takes the CSV dump of that database table, and performs various sorting and analysis operations to provide
valuable insights for content marketing and content architecture optimization. That's the goal, we're not quite there
yet.

Imagine you have a CSV that looks like this:

```CSV
"id","time","query","url","hits","user","email","temp_id","target","ip"
"1","2021-04-21 15:29:24","graphical abstract","https%3A%2F%2Fsourcewebsite.com%2F","8",100,"user@email.com,"7d8bc4df84",,"100.100.100.100"
"2","2021-04-21 15:42:56","short biography","https%3A%2F%2Fsourcewebsite.com%2Fproduction","0",,,"674facf7f7","https://sourcewebsite.com/target-page","200.200.200.200"
"3","2021-04-21 15:43:01","biography","https%3A%2F%2Fsourcewebsite.com%2F%3Fs%3Dshort%2Bbiography","4",,,"cd41fc6140","https://sourcewebsite.com/target-page","300.300.300.300"
"4","2021-04-21 15:44:05","data sharing","https%3A%2F%2Fsourcewebsite.com%2F","46",,,"079bd41fca","https://sourcewebsite.com/target-page","400.400.400.400"
```

But instead of 4 rows of data, you have hundreds of thousands, perhaps millions. All of that information will be looked
at, filtered, sorted, stored, and output in new collections.

Not all of those columns of data are useful for insight generation though, so we need the following five:
`"time","query","url","hits","target"`

## High level overview

    - keyword search
    - pull up statistics for a selected keyword
    - data cleaning
      - [done] exclude data points / columns
      - [wip] get rid of bot searches, hacking attempts
          - [wip] sql-like commands
          - [done] {search_term_string}
      - [done] string cutoff at length X
      - [done] exclude logged in user searches
      - [done] decode urls
      - [done] remove duplicates
      - [done] "+" character in "keyword" translates to whitespace
    - [wip] produce fresh datasheet for cleaned data

## Initial MVP objectives

    - find most searched keywords
    - order output by most searched

## What did I learn so far

    - many don't know how to search for things
    - many can't type
    - many use the search bar like google
    - a shocking amount of bot searches and/or hacking attempts happen
    - we need better search behaviour on the sites
        - we should deal with potential duplicates:
            - Author contribution,1
            - Author contributions,3
        - think: based on closest match patterns, qcop can show closest matches if 0 hits are returned
    - clean search queries even more at the source
        - strip irrelevant patterns within the search request so better results can be found on site