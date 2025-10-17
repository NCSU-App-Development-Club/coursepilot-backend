import aiohttp
from fastapi import FastAPI, HTTPException

from src.course_parser import CourseParser
from src.prof_parser import ProfParser

app = FastAPI()
http_client = aiohttp.ClientSession()

course_parser = CourseParser(http_client)
prof_parser = ProfParser(http_client)

@app.get("/")
async def root():
    return {"message": "Welcome to the Course API"}

@app.get("/courses")
async def get_course_prefixes():
    return await course_parser.get_course_prefixes()


@app.get("/courses/{prefix}")
async def get_courses_by_prefix(prefix: str):
    courses = await course_parser.get_courses_by_prefix(prefix)
    if not courses:
        raise HTTPException(status_code=404, detail="Prefix not found")
    return courses


@app.get("/courses/{prefix}/{number}")
async def get_course_info(prefix: str, number: str):
    course_info = await course_parser.get_course_info(prefix, number)
    if not course_info:
        raise HTTPException(status_code=404, detail="Course not found")
    return course_info


@app.get("/courses/{prefix}/{number}/{semester}")
async def get_course_sections(prefix: str, number: str, semester: str):
    sections = await course_parser.get_course_sections(prefix, number, semester)
    if sections is None:
        raise HTTPException(status_code=404, detail="Course sections not found for this semester")
    return sections


@app.get("/semesters")
async def get_semesters():
    return await course_parser.get_semesters()


@app.get("/professors/{name}")
async def get_professor_info(name: str):
    professor = await prof_parser.get_professor(name)
    if not professor:
        raise HTTPException(status_code=404, detail="Professor not found")
    return professor
