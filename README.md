# qcop search results analyzer | WORK IN PROGRESS!

## high level overview
    - keyword search
    - pull up statistics for a selected keyword
    - data cleaning
      - **[done]** exclude data points / columns
      - **[wip]** get rid of bot searches, hacking attempts
          - **[wip]** sql-like commands
          - **[done]** {search_term_string}
      - **[done]** string cutoff at length X
      - **[done]** exclude logged in user searches
      - **[done]** decode urls
      - **[done]** "+" character in "keyword" translates to whitespace
    - **[wip]** produce fresh datasheet for cleaned data

## initial objectives
    - find most searched for keywords
    - order output by most searched
    - write unit tests

## what did i learn
    - many don't know how to search for things
    - many can't type
    - many use the search bar like google
        - we need better search
            - Author contribution,1
            - Author contributions,3
    - think about it: based on closest match patterns, qcop can show closest matches if 0 hits returned
    - clean search queries even more at the source

---

A query has the following fields stored in a CSV row:

`"id","time","query","url","hits","user","email","temp_id","target","ip"`

**Example:**

`"132","2018-08-21 17:19:46","sharing your work","https%3A%2F%2Fsourceurl.com%2F","32","searcher@email.com","searcher@email.com","cc140ddc0d"," https://targeturl.com/sharing-your-work"`

We only need 5 of them:

`"time","query","url","hits","target"`

---

CleanRecord to lose "keyword" ? and rename to CleanRecordMeta?

Remove duplicates - example:

`"210645","2021-09-20 10:33:09","journal of strategic studies",,"1",,,"1fd20dcc4a",,"xxx.xx.xx.xx"`
`"210646","2021-09-20 10:33:09","journal of strategic studies",,"1",,,"1fd20dcc4a",,"xxx.xx.xx.xx"`
