use lazy_static::lazy_static;
use scraper::Selector;

lazy_static! {
    pub static ref COURSE_SELECTOR: Selector = Selector::parse("section.course").unwrap();
    pub static ref SMALL_SELECTOR: Selector = Selector::parse("small").unwrap();
    pub static ref UNITS_SELECTOR: Selector = Selector::parse("span.units").unwrap();
    pub static ref PARAGRAPH_SELECTOR: Selector = Selector::parse("p").unwrap();
    pub static ref SECTION_SELECTOR: Selector = Selector::parse("tr").unwrap();
    pub static ref ABBREVIATION_SELECTOR: Selector = Selector::parse("abbr").unwrap();
    pub static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
}
