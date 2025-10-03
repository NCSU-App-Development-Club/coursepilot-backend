# coursepilot-backend

This is the backend for the CoursePilot application.
It uses a combination of FastAPI (for HTTP requests) and
BeautifulSoup (for scraping websites).

## Setup

1. Make sure you have Python 3.13+ installed.
2. Clone this repository.
3. Create a virtual environment: `python -m venv venv` (unless your IDE does this for you)
4. Activate the virtual environment (unless your IDE does this for you):
   - On Windows: `venv\Scripts\activate`
   - On macOS/Linux: `source venv/bin/activate`
5. Install dependencies: `pip install -r requirements.txt`
6. Run the application: `fastapi dev -e src.main:app --reload` It'll reload on code changes.

Please work in your own branch and open a PR when you're ready for review. Eventually, the main
branch will automatically deploy a container to some cloud service.

View the legacy Rust implementation [here](https://github.com/IAmThe2ndHuman/coursepilot-backend-legacy).

## Get Started

I've found that looking through the Javascript of these websites is super helpful.

https://webappprd.acs.ncsu.edu/php/coursecat/directory.php
https://webappprd.acs.ncsu.edu/php/coursecat/index.php

For example, the `/directory.php` page has a function `getAllSubjects()` which
calls POST `/subjects.php` with the body `strm=all` to get all course prefixes.

By performing the following call...

```http request
POST https://webappprd.acs.ncsu.edu/php/coursecat/subjects.php
Accept: application/json
Content-Type: application/x-www-form-urlencoded

strm=all
```

...we get a response like this:

```json
{
  "subj_js": "[\"AA - Advanced Analytics\", \"ACC - Accounting\", \"ADN - Art and Design\", \"AEC - App...
}
```

Looks like there is more JSON within the `subj_js` field. We'll have to parse that out, then
deserialize it into our own data structures.

As you can see, we can find some data that is already in JSON format. Just imagine how much
info we can get if we can find more endpoints like this!

If there is no such luck, we can use BeautifulSoup to parse the HTML and extract the data we need.
But that should be a last resort.

_Also, sidenote: we should probably cache endpoints like this one, but we can worry about that
later!_

You can view the JS of the websites using Chrome DevTools (sources tab). You can also place breakpoints
to trace the code execution easier.



## Scope

The current roadmap and scope of the backend is as follows:

- given a course prefix and number and a semester, return a list of its sections
- given a course prefix, return a list of courses associated with it
    - one can simply look up the sections using one of the methods above if needed
    - one should be able to filter by grad or undergrad courses
- there should be an endpoint to return all available course prefixes
- there should be an endpoint to return all available semesters
- given a professor's name, return their ratings from RateMyProfessors

What stretch goals should we have?

## Endpoint Modeling

### GET `/courses`

Returns a list of course prefixes. Something like this:

```json
[
  {
    "prefix": "CSC",
    "name": "Computer Science"
  },
  {
    "prefix": "HESF",
    "name": "Health Exercise Studies Fitness"
  },
  ...
]
```

A stret

### GET `/courses/{prefix}`

Returns a list of courses associated with that prefix. So for `/courses/HESF` you might return:

```json
[
  {
    "prefix": "HESF",
    "number": "100",
    "name": "Cross Training"
  },
  {
    "prefix": "HESF",
    "number": "101",
    "name": "Fitness and Wellness"
  },
  ...
]
```

404 if not found.

### GET `/courses/{prefix}/{number}`

Returns course information. Example for `/courses/HESF/100`:

```json
{
  "prefix": "HESF",
  "number": "100",
  "name": "Cross Training",
  "description": "This course is designed to teach and apply the principles of...",
  "units": 2,
  "semesters": [
    "spring2025",
    "fall2025"
  ]
}
```

404 if not found.

### GET `/courses/{prefix}/{number}/{semester}`

Returns sections of this course in this semester. Example for `/courses/HESF/100/fall2026`:

```json
[
  {
    "course_prefix": "HESF",
    "course_number": "100",
    "number": "002",
    "component": "Phy",
    "availability": {
      "status": "open",
      "openSeats": 1,
      "maxSeats": 26,
      "waitlisted": 0
    },
    "schedule": {
      "days": [
        "M",
        "W",
        "F"
      ],
      "from_time": "08:30",
      "to_time": "09:20"
    },
    "location": "2615 Carmichael Gym",
    "instructors": [
      "Joanna Smith"
    ],
    "begin": "2026-08-18T00:00:00",
    "end": "2026-12-02T00:00:00",
    "restrictions": [
      "Instructor Approval Required"
    ]
  },
  ...
]
```

Return 404 if not found.

The `schedule` field can be `null` for DE classes (is that a good idea?)

We need to also figure out how to account for recitation and lab sections.
Then there's the silly physics courses that have a different block on Tuesday (exam block)

### GET `/semesters`

Returns valid semesters.

```json
[
  {
    "name": "Fall 2026",
    "code": "fall2026"
  },
  ...
]
```

### GET `/professor/{name}`

Returns RateMyProfessors ratings for a given professor name. I think there is a
library for this, but if not we can scrape the site.

```json
{
  "name": "John Doe",
  "department": "Computer Science",
  "school": "North Carolina State University",
  "num_ratings": 45,
  "overall_quality": 4.2,
  "difficulty": 3.1,
  "would_take_again": 85,
  "link": "https://www.ratemyprofessors.com/professor/1234567"
}
```