# QCop search results analyzer

## high level overview

- keyword search
    - pull up statistics for a selected keyword
- data cleaning
    - [done] exclude data points / columns
    - get rid of bot searches, hacking attempts
        - sql-like commands
        - [done] {search_term_string}
    - [done] string cutoff at length X
    - [done] exclude logged in user searches
    - [done] decode urls
    - [done] "+" character in "keyword" translates to whitespace
- produce fresh datasheet for cleaned data

## initial objectives
    - find most searched for keywords

## what did i learn
    - many don't know how to search for things
    - many can't type

A query has the following fields stored in a CSV row:

`"id","time","query","url","hits","user","email","temp_id","target","ip"`

**Example:**

"132","2018-08-21 17:19:46","sharing your work","https%3A%2F%2Fauthorservices.taylorandfrancis.com%2F","32","searcher@email.com","searcher@email.com","cc140ddc0d"," https://authorservices.taylorandfrancis.com/sharing-your-work"

We only need 5 of them:

`"time","query","url","hits","target"`

#### Things to ignore

- /Core/File/uploadPictureBase64.html
- metrics/feed/rss2/Leisure/gtm.start/gtm.start
- /index/\\think\\app/invokefunction
- Home/\\think\\app/invokefunction
- http://
- xmlrpc.php
- wp-admin/install.php
