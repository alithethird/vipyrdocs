"""Test cases for multiline docstring support."""
from ruff_docstrings_complete._core import docstring


def test_arrange_act_assert_multiline():
    """Test arrange/act/assert pattern with multiline descriptions."""
    lines = [
        "arrange: This is a very important part of a very important test so",
        "    the arrange sentence has to be loooong.",
        "act: Do the test.",
        "assert: It better not fail.",
    ]
    
    sections = docstring._get_sections(lines)
    
    # Should have one section named 'arrange' with 'act' and 'assert' as subsections
    assert len(sections) == 1
    assert sections[0].name == "arrange"
    assert len(sections[0].subs) == 2
    assert "act" in sections[0].subs
    assert "assert" in sections[0].subs
    print("✓ arrange/act/assert multiline test passed")


def test_given_when_then_multiline():
    """Test given/when/then pattern with multiline descriptions."""
    lines = [
        "given: A complex setup scenario that requires multiple lines",
        "    to properly explain all the preconditions.",
        "when: The user performs an action.",
        "then: The system should respond appropriately with",
        "    the expected behavior.",
    ]
    
    sections = docstring._get_sections(lines)
    
    assert len(sections) == 1
    assert sections[0].name == "given"
    assert len(sections[0].subs) == 2
    assert "when" in sections[0].subs
    assert "then" in sections[0].subs
    print("✓ given/when/then multiline test passed")


def test_args_section_multiline():
    """Test Args section with multiline parameter descriptions."""
    lines = [
        "Args:",
        "    param1: This is a very long description that needs to",
        "        span multiple lines because it's important.",
        "    param2: Short description.",
        "    param3: Another long description that also",
        "        needs multiple lines.",
    ]
    
    sections = docstring._get_sections(lines)
    
    assert len(sections) == 1
    assert sections[0].name == "Args"
    assert len(sections[0].subs) == 3
    assert "param1" in sections[0].subs
    assert "param2" in sections[0].subs
    assert "param3" in sections[0].subs
    print("✓ Args multiline test passed")


def test_multiple_continuation_lines():
    """Test subsections with multiple continuation lines."""
    lines = [
        "Args:",
        "    param1: This is line 1",
        "        This is line 2 of the same parameter",
        "        And this is line 3",
        "        And line 4",
        "    param2: Another parameter.",
    ]
    
    sections = docstring._get_sections(lines)
    
    assert len(sections) == 1
    assert sections[0].name == "Args"
    assert len(sections[0].subs) == 2
    assert "param1" in sections[0].subs
    assert "param2" in sections[0].subs
    print("✓ Multiple continuation lines test passed")


if __name__ == "__main__":
    test_arrange_act_assert_multiline()
    test_given_when_then_multiline()
    test_args_section_multiline()
    test_multiple_continuation_lines()
    print("\n✅ All Python integration tests passed!")
