from __future__ import annotations
from typing import Union, BinaryIO
from pathlib import Path
from .megalinref import Lookup
from ._download_fresh_data_as_json import download_fresh_data_as_json

def open_binary_file(file: Union[str, Path, BinaryIO, bytes, bytearray]) -> Lookup:
    """
    Opens a binary file and creates a Lookup object from its contents.
    
    Parameters:
    file: Can be a file path (str), a file handle (file-like object) or a bytes object.

    Returns:
    Lookup: A Lookup object created from the binary content of the file.
    
    Raises:
    TypeError: If the provided 'file' parameter is not a str, file-like object, or bytes.
    FileNotFoundError: If the provided 'file' parameter is a str (file path) and the file does not exist.
    """
    if isinstance(file, (str, Path)):  # for file paths
        path_to_data = Path(file)
        with path_to_data.open("rb") as bin_cache:
            slk_lookup = Lookup.from_binary(bin_cache.read())
        return slk_lookup
    elif isinstance(file, bytes):  # for bytes or bytearray objects
        return Lookup.from_binary(file)
        # TODO: Test if possible
    elif isinstance(file, bytearray):  # for bytes or bytearray objects
        return Lookup.from_binary(bytes(file))
        # TODO: Test if possible
    elif hasattr(file, 'read'):  # for file-like objects
        return Lookup.from_binary(file.read())
    else:
        raise TypeError(f"Expected str, bytes, or file-like object, got {type(file).__name__}")

def open_from_cache_or_download(file_path: Union[Path, str]) -> Lookup:
    """
    Opens a binary file at the provided path and creates a Lookup object from
    its contents. If the file does not exist, downloads fresh data, converts it
    to binary and caches it.

    Parameters:
    file_path: A file path (str).

    Returns:
    Lookup: A Lookup object created from the binary content of the file.

    Raises:
    TypeError: If the provided 'file_path' parameter is not a str.
    """

    path_to_data = Path(file_path)

    try:
        return open_binary_file(path_to_data)
    except FileNotFoundError:
        print(f"There is no data cached at {path_to_data}")
        # Fetch new data
        downloaded_json_data = download_fresh_data_as_json()
        # Convert to binary
        slk_lookup = Lookup.from_dict(downloaded_json_data)

        try:
            # Cache the data
            with path_to_data.open("wb") as f:
                f.write(slk_lookup.to_binary())
        except Exception as e:
            print(f"Failed to cache data due to error. It will be downloaded again next time.: {e}")
            return slk_lookup
    except Exception as e:
        print("Unknown error while trying to open the cache")
        raise e
