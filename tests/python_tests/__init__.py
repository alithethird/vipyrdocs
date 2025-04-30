"""Tests."""

from ruff_docstrings_complete._core import constants

ERROR_CODE_PREFIX = constants.ERROR_CODE_PREFIX
MORE_INFO_BASE = constants.MORE_INFO_BASE


DOCSTR_MISSING_CODE = f"{ERROR_CODE_PREFIX}010"
DOCSTR_MISSING_MSG = (
    f"{DOCSTR_MISSING_CODE} docstring should be defined for a function/ method/ class"
    f"{MORE_INFO_BASE}{DOCSTR_MISSING_CODE.lower()}"
)
RETURNS_SECTION_NOT_IN_DOCSTR_CODE = f"{ERROR_CODE_PREFIX}030"
RETURNS_SECTION_NOT_IN_DOCSTR_MSG = (
    f"{RETURNS_SECTION_NOT_IN_DOCSTR_CODE} function/ method that returns a value should have the "
    f"returns section in the docstring{MORE_INFO_BASE}{RETURNS_SECTION_NOT_IN_DOCSTR_CODE.lower()}"
)
RETURNS_SECTION_IN_DOCSTR_CODE = f"{ERROR_CODE_PREFIX}031"
RETURNS_SECTION_IN_DOCSTR_MSG = (
    f"{RETURNS_SECTION_IN_DOCSTR_CODE} function/ method that does not return a value should not "
    f"have the returns section in the docstring"
    f"{MORE_INFO_BASE}{RETURNS_SECTION_IN_DOCSTR_CODE.lower()}"
)
MULT_RETURNS_SECTIONS_IN_DOCSTR_CODE = f"{ERROR_CODE_PREFIX}032"
MULT_RETURNS_SECTIONS_IN_DOCSTR_MSG = (
    f"{MULT_RETURNS_SECTIONS_IN_DOCSTR_CODE} a docstring should only contain a single returns "
    "section, found %s"
    f"{MORE_INFO_BASE}{MULT_RETURNS_SECTIONS_IN_DOCSTR_CODE.lower()}"
)
YIELDS_SECTION_NOT_IN_DOCSTR_CODE = f"{ERROR_CODE_PREFIX}040"
YIELDS_SECTION_NOT_IN_DOCSTR_MSG = (
    f"{YIELDS_SECTION_NOT_IN_DOCSTR_CODE} function/ method that yields a value should have the "
    f"yields section in the docstring{MORE_INFO_BASE}{YIELDS_SECTION_NOT_IN_DOCSTR_CODE.lower()}"
)
YIELDS_SECTION_IN_DOCSTR_CODE = f"{ERROR_CODE_PREFIX}041"
YIELDS_SECTION_IN_DOCSTR_MSG = (
    f"{YIELDS_SECTION_IN_DOCSTR_CODE} function/ method that does not yield a value should not "
    f"have the yields section in the docstring"
    f"{MORE_INFO_BASE}{YIELDS_SECTION_IN_DOCSTR_CODE.lower()}"
)
MULT_YIELDS_SECTIONS_IN_DOCSTR_CODE = f"{ERROR_CODE_PREFIX}042"
MULT_YIELDS_SECTIONS_IN_DOCSTR_MSG = (
    f"{MULT_YIELDS_SECTIONS_IN_DOCSTR_CODE} a docstring should only contain a single yields "
    "section, found %s"
    f"{MORE_INFO_BASE}{MULT_YIELDS_SECTIONS_IN_DOCSTR_CODE.lower()}"
)