# coursepilot-backend

Backend for CoursePilot (Flutter Team Project) built in Rust.

## TODO

Things to be implemented

### Parsing
Parsing course catalog from the course catalog website.

- [x] Searching for courses
- [x] Searching for a single course
- [ ] Searching for available terms
- [ ] Parsing courses from HTML response [in progress]
- [ ] Waitlist parsing
- [ ] Parsing terms from HTML response
- [ ] Serialization of courses and terms to JSON
- [ ] (possibly) DB caching

### API
This is after parsing is implemented.

- [ ] Refactor parsing crate to workspace
- [ ] Set parsing crate as library crate
- [ ] Create API sub-crate and set as binary crate

### Misc.
Miscellaneous tasks

- [ ] Clean up imports
- [ ] Refactor `html.rs` to propagate errors from decoding process instead of generic MalformedCourse
- [ ] Add documentation
- [ ] Set up CI/CD and containerization
- [ ] Set up logging
- [ ] Probably something else I'm forgetting