from datetime import datetime
from typing import Optional

from aiohttp import ClientSession

from src.models import CoursePrefixListing, CourseListing, CourseDetail, Semester, SectionDetail, Availability, Schedule


class CourseParser:
    def __init__(self, client: ClientSession):
        self._client = client
        """Use this client to make HTTP requests to the course catalog system."""

    async def get_course_prefixes(self) -> list[CoursePrefixListing]:
        pass

    async def get_courses_by_prefix(self, prefix: str) -> list[CourseListing]:
        pass

    async def get_course_info(self, prefix: str, number: str) -> Optional[CourseDetail]:
        pass

    async def get_course_sections(
        self, prefix: str, number: str, semester: str
    ) -> Optional[list[SectionDetail]]:
        pass

    async def get_semesters(self) -> list[Semester]:
        pass


class MockCourseParser:
    """A mock parser that returns hardcoded data for API endpoints. Yes the data is AI generated."""

    _PREFIXES = [
            CoursePrefixListing(prefix="CSC", name="Computer Science"),
            CoursePrefixListing(prefix="HESF", name="Health Exercise Studies Fitness"),
            CoursePrefixListing(prefix="MA", name="Mathematics")
    ]

    _COURSE_LISTINGS = [
            CourseListing(prefix="CSC", number="316", name="Data Structures and Algorithms"),
            CourseListing(prefix="CSC", number="226", name="Discrete Mathematics"),
            CourseListing(prefix="HESF", number="100", name="Cross Training"),
            CourseListing(prefix="HESF", number="101", name="Fitness and Wellness"),
            CourseListing(prefix="MA", number="241", name="Calculus II"),
            CourseListing(prefix="MA", number="242", name="Calculus III")
    ]

    _COURSE_INFOS = [
        CourseDetail(prefix="CSC", number="316", name="Data Structures and Algorithms", description="""This is a very long description of the course.
        
It goes on for multiple lines to test the API's ability to handle long text fields. The course covers various data structures such as lists, stacks, queues, trees, and graphs. It also delves into
algorithms for sorting, searching, and traversing these data structures. Students will learn about algorithm complexity and optimization techniques. The course includes both theoretical concepts and practical implementations in programming assignments and projects.
        """, units=4, semesters=["fall2026"]),
        CourseDetail(prefix="CSC", number="226", name="Discrete Mathematics", description="An introduction to discrete mathematics, including logic, set theory, combinatorics, graph theory, and algorithms.", units=4, semesters=["spring2026", "fall2026"]),
        CourseDetail(prefix="HESF", number="100", name="Cross Training", description="A fitness course focusing on cross-training techniques to improve overall physical fitness and endurance.", units=1, semesters=["fall2026"]),
        CourseDetail(prefix="HESF", number="101", name="Fitness and Wellness", description="A course that provides students with the knowledge and skills to lead a healthy lifestyle.", units=1, semesters=["spring2026", "fall2026"]),
        CourseDetail(prefix="MA", number="241", name="Calculus II", description="Second course in a three-course calculus sequence. Covers techniques and applications of integration, infinite series, and polar and parametric equations.", units=4, semesters=["spring2026", "fall2026"]),
        CourseDetail(prefix="MA", number="242", name="Calculus III", description="Third course in a three-course calculus sequence. Covers vectors, vector-valued functions, partial derivatives, multiple integrals, and vector calculus.", units=4, semesters=["fall2026"])
    ]

    _SECTIONS = {
        "fall2026": [
            SectionDetail(
                course_prefix="CSC", course_number="316",
                number="001", component="LEC",
                availability=Availability(status="open", openSeats=10, maxSeats=150, waitlisted=0),
                schedule=Schedule(days=["M", "W"], from_time="13:30", to_time="14:45"),
                location="2200 EB2", instructors=["Bob Reaves"],
                begin=datetime(2026, 8, 19), end=datetime(2026, 12, 4),
                restrictions=[],
            ),
            SectionDetail(
                course_prefix="CSC", course_number="316",
                number="002", component="LEC",
                availability=Availability(status="waitlist", openSeats=0, maxSeats=150, waitlisted=5),
                schedule=Schedule(days=["T", "TH"], from_time="10:15", to_time="11:30"),
                location="1231 EB2", instructors=["E. F. Gehringer"],
                begin=datetime(2026, 8, 19), end=datetime(2026, 12, 4),
                restrictions=[],
            ),
            SectionDetail(
                course_prefix="CSC", course_number="316",
                number="201", component="LAB",
                availability=Availability(status="open", openSeats=5, maxSeats=30, waitlisted=0),
                schedule=Schedule(days=["F"], from_time="10:40", to_time="12:30"),
                location="1231 EB2", instructors=["Staff"],
                begin=datetime(2026, 8, 19), end=datetime(2026, 12, 4),
                restrictions=[],
            ),
            SectionDetail(
                course_prefix="CSC", course_number="226",
                number="001", component="LEC",
                availability=Availability(status="open", openSeats=25, maxSeats=200, waitlisted=0),
                schedule=Schedule(days=["T", "TH"], from_time="13:30", to_time="14:45"),
                location="102 Dabney", instructors=["Jacob Gardner"],
                begin=datetime(2026, 8, 19), end=datetime(2026, 12, 4),
                restrictions=[],
            ),
            SectionDetail(
                course_prefix="HESF", course_number="100",
                number="002", component="Phy",
                availability=Availability(status="open", openSeats=1, maxSeats=26, waitlisted=0),
                schedule=Schedule(days=["M", "W", "F"], from_time="08:30", to_time="09:20"),
                location="2615 Carmichael Gym", instructors=["Joanna Stegall"],
                begin=datetime(2026, 8, 18), end=datetime(2026, 12, 2),
                restrictions=["Instructor Approval Required"],
            ),
            SectionDetail(
                course_prefix="HESF", course_number="101",
                number="001", component="Phy",
                availability=Availability(status="closed", openSeats=0, maxSeats=30, waitlisted=10),
                schedule=Schedule(days=["T", "TH"], from_time="11:20", to_time="12:10"),
                location="1306 Carmichael Gym", instructors=["Staff"],
                begin=datetime(2026, 8, 18), end=datetime(2026, 12, 2),
                restrictions=[],
            ),
            SectionDetail(
                course_prefix="MA", course_number="241",
                number="001", component="LEC",
                availability=Availability(status="open", openSeats=15, maxSeats=40, waitlisted=0),
                schedule=Schedule(days=["M", "W", "F"], from_time="09:35", to_time="10:25"),
                location="2102 SAS Hall", instructors=["Staff"],
                begin=datetime(2026, 8, 19), end=datetime(2026, 12, 4),
                restrictions=[],
            ),
            SectionDetail(
                course_prefix="MA", course_number="242",
                number="003", component="LEC",
                availability=Availability(status="open", openSeats=3, maxSeats=40, waitlisted=0),
                schedule=Schedule(days=["T", "TH"], from_time="16:30", to_time="17:45"),
                location="3214 SAS Hall", instructors=["Staff"],
                begin=datetime(2026, 8, 19), end=datetime(2026, 12, 4),
                restrictions=[],
            )
        ],
        "spring2026": [
            SectionDetail(
                course_prefix="CSC", course_number="226",
                number="001", component="LEC",
                availability=Availability(status="open", openSeats=30, maxSeats=200, waitlisted=0),
                schedule=Schedule(days=["M", "W", "F"], from_time="15:00", to_time="16:15"),
                location="216 Mann Hall", instructors=["D. E. Couto"],
                begin=datetime(2026, 1, 7), end=datetime(2026, 4, 22),
                restrictions=[],
            ),
            SectionDetail(
                course_prefix="HESF", course_number="101",
                number="003", component="Phy",
                availability=Availability(status="open", openSeats=5, maxSeats=30, waitlisted=0),
                schedule=Schedule(days=["M", "W"], from_time="09:35", to_time="10:25"),
                location="1306 Carmichael Gym", instructors=["Staff"],
                begin=datetime(2026, 1, 6), end=datetime(2026, 4, 21),
                restrictions=[],
            ),
            SectionDetail(
                course_prefix="MA", course_number="241",
                number="002", component="LEC",
                availability=Availability(status="waitlist", openSeats=0, maxSeats=40, waitlisted=8),
                schedule=Schedule(days=["T", "TH"], from_time="08:30", to_time="09:45"),
                location="1108 SAS Hall", instructors=["Staff"],
                begin=datetime(2026, 1, 7), end=datetime(2026, 4, 22),
                restrictions=[],
            )
        ]
    }

    _SEMESTERS = [
        Semester(name="Fall 2026", code="fall2026", _internal_code="2268"),
        Semester(name="Spring 2026", code="spring2026", _internal_code="2262"),
    ]

    async def get_course_prefixes(self) -> list[CoursePrefixListing]:
        return self._PREFIXES

    async def get_courses_by_prefix(self, prefix: str) -> list[CourseListing]:
        return [course for course in self._COURSE_LISTINGS if course.prefix == prefix]


    async def get_course_info(self, prefix: str, number: str) -> Optional[CourseDetail]:
        for course in self._COURSE_INFOS:
            if course.prefix == prefix and course.number == number:
                return course
        return None


    async def get_course_sections(
        self, prefix: str, number: str, semester: str
    ) -> Optional[list[SectionDetail]]:
        if semester not in self._SECTIONS:
            return None

        return [
            section for section in self._SECTIONS[semester]
            if section.course_prefix == prefix and section.course_number == number
        ]


    async def get_semesters(self) -> list[Semester]:
        return self._SEMESTERS
