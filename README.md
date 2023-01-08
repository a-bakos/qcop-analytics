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
    - order output by most searched

## what did i learn
    - many don't know how to search for things
    - many can't type
    - many use the search bar like google
        - we need better search
    - think about it: based on closest match patterns, qcop can show closest matches if 0 hits returned
    - clean search queries even more

A query has the following fields stored in a CSV row:

`"id","time","query","url","hits","user","email","temp_id","target","ip"`

**Example:**

"132","2018-08-21 17:19:46","sharing your work","https%3A%2F%2Fauthorservices.taylorandfrancis.com%2F","32","searcher@email.com","searcher@email.com","cc140ddc0d"," https://authorservices.taylorandfrancis.com/sharing-your-work"

We only need 5 of them:

`"time","query","url","hits","target"`

2000&#101;&#110;&#099;&#116;&#101;&#115;&#116;&#120;&#074;&#068;&#066;&#073;,1
%2529%2520AND%252038%2520like%252038%2520OR%25201234%2520like%2520%25287279477,1
2021\\x5c\&quot;yelpcSS,1
2000&#039;zGrnjITQMQ&#039;foo,1
so89c0714bc0a5e405aso81591900432so8
%ef%bc%b3%e2%84%aa%ef%bd%85%ef%bd%8e%ef%bd%83%ef%bc%af%ef%bc%ac%ef%bd%89%ef%bc%a8%ef%bc%a9,1
%2Ax%2A/%7C%7C%28DBMS_PIPE.RECEIVE_MESSAGE%28CHR%2841%29%2C10%29%29/%2Ax%2A/%7C%7C,1
\&#039;\&quot;&gt;&lt;YGMLHbD&gt;&lt;/YGMLHbD&gt;,1
admin/\\think\\Request/input,5
\&quot;1\&quot;IHOHMoAmri\&quot;foo,1
/home/service/index/id/1\&#039;,2
%65%6e%63%69%6f%44%47%49,1
\\x65\\x6e\\x63\\x75\\x68\\x6f\\x61\\x47,1
() { :;}; printf \&quot;detection[%s]string\&quot; \&quot;CqrjIlFYXQIkXsH\&quot;,1
%2529%2520AND%252078%2520like%252078%2520OR%25201234%2520like%2520%25287276524,1
2000&amp;sleep 1&amp;\&#039;\\\&quot;`0&amp;sleep 1&amp;`\&#039;,16



;foo
/home
()
printf
