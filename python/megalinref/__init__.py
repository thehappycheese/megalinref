from .megalinref import (
    SLKLookup,
    Cwy as _internal_cwy,
    NetworkType as _internal_network_type,
)
from ._data_handling import download_fresh_data_as_json

Cwy:dict[str, int] = {
    "Left":   _internal_cwy["Left"],
    "Single": _internal_cwy["Single"],
    "Right":  _internal_cwy["Right"],
    "L":      _internal_cwy["Left"],
    "S":      _internal_cwy["Single"],
    "R":      _internal_cwy["Right"],

    "All":    (
        _internal_cwy["Left"] |
        _internal_cwy["Single"] |
        _internal_cwy["Right"]
    ),
}
"""
Carriageway enumeration used to filter the 
road network data in the `SLKLookup.lookup()` method.

Values in this dictionary are integers that can be
combined using bitwise operators.


```python
from megalinref import SLKLookup, Cwy

sl = SLKLookup(...)

sl.lookup(
    ...,
    cwy = Cwy["Left"] | Cwy["Right"],
)
```
"""



NetworkType:dict[str, int] = {
    "State Road":                 _internal_network_type["State Road"],
    "Local Road":                 _internal_network_type["Local Road"],
    "Miscellaneous Road":         _internal_network_type["Miscellaneous Road"],
    "Main Roads Controlled Path": _internal_network_type["Main Roads Controlled Path"],
    "Proposed Road":              _internal_network_type["Proposed Road"],
    "Crossover":                  _internal_network_type["Crossover"],

    "All":(
        _internal_network_type["State Road"] |
        _internal_network_type["Local Road"] |
        _internal_network_type["Miscellaneous Road"] |
        _internal_network_type["Main Roads Controlled Path"] |
        _internal_network_type["Proposed Road"] |
        _internal_network_type["Crossover"]
    )
}
"""
NetworkType enumeration used to filter the
road network data in the `SLKLookup.lookup()` method.

```python
from megalinref import SLKLookup, NetworkType

sl = SLKLookup(...)

sl.lookup(
    ...,
    network_type = NetworkType["State Road"] | NetworkType["Proposed Road"],
)
```
"""