"""The attributes section checks."""


def is_property_decorator(node: ast.expr) -> bool:
    """Docstring here

    Returns:
        true if its property decorator
    """
    if isinstance(node, ast.Name):
        return node.id in {"property", "cached_property"}

    # Handle call
    if isinstance(node, ast.Call):
        return is_property_decorator(node=node.func)

    # Handle attr
    if isinstance(node, ast.Attribute):
        value = node.value
        return (
            node.attr == "cached_property"
            and isinstance(value, ast.Name)
            and value.id == "functools"
        )

    # There is no valid syntax that gets to here
    return False  # pragma: nocover

def erty_decorator(node: ast.expr) -> bool:
    """Docstring here
    
    Args:
        node: some
    """

    if isinstance(node, ast.Name):
        return node.id in {"property", "cached_property"}

    # Handle call
    if isinstance(node, ast.Call):
        return is_property_decorator(node=node.func)

    # Handle attr
    if isinstance(node, ast.Attribute):
        value = node.value
        return (
                node.attr == "cached_property"
                and isinstance(value, ast.Name)
                and value.id == "functools"
        )

    # There is no valid syntax that gets to here
    return False  # pragma: nocover

