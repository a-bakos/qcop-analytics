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
    - many use the search bar like google
    - think about it: based on closest match patterns, qcop can show closest matches if 0 hits returned
    - clean search queries even more

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


- 1\&#039;%20AND%20100%20like%20101%20OR%20\&#039;1234\&#039;%20like%20\&#039;7271704,1
- 1\&#039;%20AND%209%20like%209%20OR%20\&#039;1234\&#039;%20like%20\&#039;7275481,1
- 1\&#039;%29%20AND%2034%20like%2035%20OR%20\&#039;1234\&#039;%20like%20%28\&#039;7277891,1
- 1\&#039;%29%20AND%207%20like%207%20OR%20\&#039;1234\&#039;%20like%20%28\&#039;7274903,1
- 1\&#039;%22%3e%3czJNneTA%3e%3c/zJNneTA%3e,1
- 1\&#039;%20AND%204%20like%204%20OR%20\&#039;1234\&#039;%20like%20\&#039;7274076,1
- 1\&#039;JyKpaivBRU\&#039;foo,1

- 1/%252f%252f%252e%252e%252fweb.config,2
- 1/1%ef%bc%82%29%20AND%2071%20like%2072%20OR%20%ef%bc%821234%ef%bc%82%20like%20%28%ef%bc%827276356,1
- 1/1\&#039;%29%20AND%2039%20like%2039%20OR%20\&#039;1234\&#039;%20like%20%28\&#039;7271941,1
- 1/%5C..%5C..%5C..%5C..%5C..%5C..%5C..%5C..%5C..%5C..%5Cwindows%5Cwin.ini/3,2
- 1//%26%23x09%3B/authorservices.taylorandfrancis.com.appcheck-ng.com/foo.js/F1000Research,1
- 1/%22page%22HRXqYOxwgQ%22foo/3,1
- 1/DS%e2%84%aaencpwpdT,1
- 1/1\&quot;%29%20AND%2076%20like%2076%20OR%20\&quot;1234\&quot;%20like%20%28\&quot;7274707,1
- 1/1\&#039;%29%20AND%2036%20like%2036%20OR%20\&#039;1234\&#039;%20like%20%28\&#039;7274378,1
- 1/1\&quot;%29%20AND%2074%20like%2074%20OR%20\&quot;1234\&quot;%20like%20%28\&quot;7273518,1
- 1/1%20AND%2030%20like%2031%20OR%201234%20like%207275182,1
- 1/file:///c:/windows/win.ini,2
- 1/%5Cx65%5Cx6e%5Cx63%5Cx69%5Cx56%5Cx65%5Cx63%5Cx49/2/F1000Research,1
- 1/%28DBMS_PIPE.RECEIVE_MESSAGE%28CHR%2841%29%2C10%29%29/30,1
- 1/1%ef%bc%87%29%20AND%209%20like%209%20OR%20%ef%bc%871234%ef%bc%87%20like%20%28%ef%bc%877271679,1
- 1/%2B%28%28SELECT%201%20FROM%20%28SELECT%20SLEEP%2810%29%29A%29%29%2B/2/F1000Research,1
- 1/1%20AND%2080%20like%2080%20OR%201234%20like%207270850,1
- 1/%28%28SELECT%201%20FROM%20%28SELECT%20SLEEP%2810%29%29A%29%29,3

- 1%ef%bc%87%20AND%209%20like%209%20OR%20%ef%bc%871234%ef%bc%87%20like%20%ef%bc%877277366,1
- 1%29%20AND%2086%20like%2087%20OR%201234%20like%20%287274711,1
- 1%ef%bc%82%29%20AND%2052%20like%2052%20OR%20%ef%bc%821234%ef%bc%82%20like%20%28%ef%bc%827271451,1
- 1%65%6e%63%74%65%73%74%55%67%49%7a%67,1
- 1%ef%bc%82%29%20AND%2042%20like%2043%20OR%20%ef%bc%821234%ef%bc%82%20like%20%28%ef%bc%827270622,1
- 1%20AND%2027%20like%2027%20OR%201234%20like%207271782,1
- 1%22%3e%3c%22%3e%3cxgSeTCC%3eabc%3c/xgSeTCC%3edef,1

- 1&amp;sleep 10&amp;\&#039;\\\&quot;`0&amp;sleep 10&amp;`\&#039;,5


