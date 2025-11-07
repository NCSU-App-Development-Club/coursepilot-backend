from datetime import datetime

from pydantic import BaseModel


class CoursePrefixListing(BaseModel):
    prefix: str
    name: str

class CourseListing(BaseModel):
    prefix: str
    number: str
    name: str

class CourseDetail(BaseModel):
    prefix: str
    number: str
    name: str
    description: str
    units: int
    semesters: list[str]

class Availability(BaseModel):
    status: str
    openSeats: int
    maxSeats: int
    waitlisted: int

class Schedule(BaseModel):
    days: list[str]
    from_time: str | None = None
    to_time: str | None = None

class SectionDetail(BaseModel):
    course_prefix: str
    course_number: str
    number: str
    component: str
    availability: Availability
    recitations: list["SectionDetail"] | None = None
    schedule: Schedule
    location: str
    instructors: list[str]
    begin: datetime
    end: datetime
    restrictions: list[str]

class Semester(BaseModel):
    code: str
    name: str
    _internal_code: str
    """This is the code used by the course catalog system. It is not intended for public viewing."""

class ProfessorDetail(BaseModel):
    name: str
    department: str
    school: str
    num_ratings: int
    overall_quality: float
    difficulty: float
    would_take_again: float
    link: str