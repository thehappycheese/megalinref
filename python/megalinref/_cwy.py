from typing import Dict
from .megalinref import Cwy as _internal_cwy
from ._download_fresh_data_as_json import download_fresh_data_as_json
from ._factory_functions import download_fresh_data_as_json, open_from_cache_or_download

Cwy:Dict[str, int] = {
    "Left":   _internal_cwy["L"],
    "Single": _internal_cwy["S"],
    "Right":  _internal_cwy["R"],
    "L":      _internal_cwy["L"],
    "S":      _internal_cwy["S"],
    "R":      _internal_cwy["R"],

    "All":    (
        _internal_cwy["L"] |
        _internal_cwy["S"] |
        _internal_cwy["R"]
    ),
}
"""
Carriageway enumeration used to filter the 
road network data in the `SLKLookup.lookup()` method.

Values in this dictionary are integers that can be
combined using bitwise operators.


```python
from megalinref import SLKLookup, Cwy

sl = Lookup(...)

sl.lookup(
    ...,
    cwy = Cwy["Left"] | Cwy["Right"],
)
```
"""

