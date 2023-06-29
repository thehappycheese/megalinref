from __future__ import annotations
from typing import Union, BinaryIO
from pathlib import Path
from .megalinref import Lookup
from ._download_fresh_data_as_json import download_fresh_data_as_json

def open_binary_file(file: Union[str, Path, BinaryIO, bytes, bytearray]) -> Lookup:
    """
    Opens a binary file and creates a Lookup object from its contents.
    
    Parameters:
    file: a path or file-like object

    Returns:
    Lookup: A Lookup object created from the binary content of the file.
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

    example:

    ```
    # the path to the cache file (name it whatever you want)
    cache_path = "megalinref_cache.bin"

    lookup = open_from_cache_or_download(cache_path)
    ```

    Parameters:
    file_path: Path the cache file that will be read or created (str, Path).

    Returns:
    Lookup: A Lookup object created from the binary content of the file.
    """

    path_to_data = Path(file_path)

    if not path_to_data.parent.exists():
        class ParentFolderDoesNotExist(Exception):
            """Exception to indicate a parent folder of a target file does not exist"""
        raise ParentFolderDoesNotExist(f"Parent directory of the target cache file does not exist. Please ensure this folder exists and try again: '{path_to_data.parent}'")

    try:
        return open_binary_file(path_to_data)
    except FileNotFoundError:
        print(f"There is no data cached at {path_to_data}")
        open_file_handel = path_to_data.open("wb")
        try:
            # Fetch new data
            downloaded_json_data = download_fresh_data_as_json()
            # Read from Dictionary
            slk_lookup = Lookup.from_dict(downloaded_json_data)
            try:
                # Convert the data to binary
                binary_data = slk_lookup.to_binary()
                # Cache the data
                open_file_handel.write(binary_data)
            except Exception as exception:
                print(f"Failed to cache data. Road Network will be downloaded again on next run;\n{exception}")
            finally:
                return slk_lookup
        finally:
            open_file_handel.close()            
    except Exception as exception:
        print("Unknown error while trying to open the cache")
        raise exception
