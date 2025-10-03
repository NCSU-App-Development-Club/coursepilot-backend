from typing import Optional

from aiohttp import ClientSession

from src.models import ProfessorDetail


class ProfParser:
    def __init__(self, client: ClientSession):
        self._client = client
        """Use this client to make HTTP requests if needed."""

    async def get_professor(self, name: str) -> Optional[ProfessorDetail]:
        pass


class MockProfParser:
    """A mock parser that returns hardcoded data for API endpoints. Yes the data is AI generated."""

    _PROFESSORS = [
        ProfessorDetail(
            name="Bob Reaves",
            department="Computer Science",
            school="NC State University",
            num_ratings=120,
            overall_quality=4.5,
            difficulty=3.2,
            would_take_again=90.0,
            link="https://www.ratemyprofessors.com/professor/123456"
        ),
        ProfessorDetail(
            name="E. F. Gehringer",
            department="Computer Science",
            school="NC State University",
            num_ratings=85,
            overall_quality=4.1,
            difficulty=3.8,
            would_take_again=87.0,
            link="https://www.ratemyprofessors.com/professor/234567"
        ),
        ProfessorDetail(
            name="Jacob Gardner",
            department="Computer Science",
            school="NC State University",
            num_ratings=40,
            overall_quality=4.0,
            difficulty=2.9,
            would_take_again=80.0,
            link="https://www.ratemyprofessors.com/professor/345678"
        ),
        ProfessorDetail(
            name="Joanna Stegall",
            department="Health Exercise Studies",
            school="NC State University",
            num_ratings=25,
            overall_quality=4.7,
            difficulty=2.5,
            would_take_again=95.0,
            link="https://www.ratemyprofessors.com/professor/456789"
        ),
        ProfessorDetail(
            name="D. E. Couto",
            department="Computer Science",
            school="NC State University",
            num_ratings=60,
            overall_quality=4.2,
            difficulty=3.0,
            would_take_again=88.0,
            link="https://www.ratemyprofessors.com/professor/567890"
        )
    ]

    async def get_professor(self, name: str) -> Optional[ProfessorDetail]:
        for prof in self._PROFESSORS:
            if prof.name == name:
                return prof
        return None
