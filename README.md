# QCop search results analyzer

## high level overview

- keyword search
    - pull up statistics for a selected keyword
- data cleaning
    - exclude data points / columns
    - get rid of bot searches, hacking attempts
    - string cutoff at length X
    - exclude logged in user searches ?
    - decode urls

## initial objectives
    - find most search for keywords

## what did i learn
    - most people can't type

## technical ideas
    - find csv parser crate

A query has the following fields stored in a CSV row:

`"id","time","query","url","hits","user","email","temp_id","target","ip"`

**Example:**

"132","2018-08-21 17:19:46","sharing your work","https%3A%2F%2Fauthorservices.taylorandfrancis.com%2F","32","searcher@email.com","searcher@email.com","cc140ddc0d"," https://authorservices.taylorandfrancis.com/sharing-your-work"

We only need 5 of them:

`"time","query","url","hits","target"`
