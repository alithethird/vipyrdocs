"""
Examples of multi-line test docstrings.

This file demonstrates the new feature that allows test docstrings to have
multi-line descriptions for arrange/act/assert or given/when/then patterns
using indentation-based continuation.
"""


def test_arrange_act_assert_pattern():
    """
    arrange: This is a very important part of a very important test so
        the arrange sentence has to be loooong and span multiple lines
        to properly describe the complex setup.
    act: Do the test.
    assert: It better not fail 😠.
    """
    # Test implementation here
    pass


def test_given_when_then_pattern():
    """
    given: A complex setup scenario that requires multiple lines
        to properly explain all the preconditions and
        initial state of the system under test.
    when: The user performs an action.
    then: The system should respond appropriately with
        the expected behavior and state changes.
    """
    # Test implementation here
    pass


def test_multiline_continuation():
    """
    arrange: First we need to set up the database with
        initial data that includes users and
        their associated permissions and
        various configuration settings that
        are necessary for this particular test scenario.
    act: Execute the migration script.
    assert: All tables should be updated correctly and
        all constraints should be in place.
    """
    # Test implementation here
    pass


def function_with_multiline_args(param1, param2, param3):
    """Function demonstrating multiline Args section.
    
    Args:
        param1: This is a very long parameter description that needs to
            span multiple lines because it describes something complex
            and important about the parameter usage.
        param2: Short description.
        param3: Another long description that also
            needs multiple lines to explain properly.
    
    Returns:
        A result value.
    """
    return param1 + param2 + param3
