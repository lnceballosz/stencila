from typing import Optional

from stencila import _stencila
from stencila.stencila_types import Node
from stencila.utilities import from_json, to_json


async def from_string(string: str, format: Optional[str] = "json") -> Node:
    """
    Decode a Stencila Schema node from a string.

    Args:
        string (str): The string to decode to a node.
        format (Optional[str]): The format to decode from. Defaults to "json".

    Returns:
        Node: A Stencila Schema node.
    """
    return from_json(await _stencila.convert.from_string(string, {"format": format}))


async def from_path(path: str, format: Optional[str] = None) -> Node:
    """
    Decode a Stencila Schema node from a filesystem path.

    Args:
        path (str): The path to decode to a node.
        format (Optional[str]): The format to decode from. If not supplied, it
            is inferred from the path.

    Returns:
        Node: A Stencila Schema node.
    """
    return from_json(await _stencila.convert.from_path(path, {"format": format}))


async def to_string(
    node: Node,
    *,
    format: Optional[str] = "json",
    standalone: bool = False,
    compact: bool = False,
) -> str:
    """
    Encode a Stencila Schema node to a string.

    Args:
        node (Node): The node to encode.
        format (Optional[str]): The format to encode to. Defaults to "json".
        standalone (bool): Whether to encode as a standalone document. Defaults
            to False.
        compact (bool): Whether to encode in compact form. Defaults to False.

    Returns:
        str: The node encoded as a string in the specified format.
    """
    return await _stencila.convert.to_string(
        to_json(node), {"format": format, "standalone": standalone, "compact": compact}
    )


async def to_path(
    node: Node,
    path: str,
    *,
    format: Optional[str] = None,
    standalone: bool = False,
    compact: bool = False,
):
    """
    Encode a Stencila Schema node to a filesystem path.

    Args:
        node (Node): The node to encode.
        path (str): The path to encode the node to.
        format (Optional[str]): The format to encode to. If not supplied, it is
            inferred from the path.
        standalone (bool): Whether to encode as a standalone document. Defaults
            to False.
        compact (bool): Whether to encode in compact form. Defaults to False.
    """

    return await _stencila.convert.to_path(
        to_json(node),
        path,
        {"format": format, "standalone": standalone, "compact": compact},
    )


async def from_to(  # noqa: PLR0913
    input: Optional[str] = None,
    output: Optional[str] = None,
    *,
    from_format=None,
    to_format=None,
    to_standalone=False,
    to_compact=False,
) -> str:
    """
    Convert a document from one format to another.

    Args:
        input (Optional[str]): The input path. If not supplied, stdin will be
            read.
        output (Optional[str]): The output path. If not supplied, the converted
            input will be returned.
        from_format (Optional[str]): The format of the input. If not supplied,
            inferred from the input path.
        to_format (Optional[str]): The format of the output. If not supplied,
            inferred from the output path.
        to_standalone (bool): Whether to encode as a standalone document.
            Defaults to False.
        to_compact (bool): Whether to encode in compact form. Defaults to
            False.

    Returns:
        str: The converted document as a string, or the path to the converted document.
    """
    return await _stencila.convert.from_to(
        input if input else "",
        output if output else "",
        {"format": from_format},
        {"format": to_format, "standalone": to_standalone, "compact": to_compact},
    )
