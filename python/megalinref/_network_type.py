from typing import Dict
from .megalinref import NetworkType as _internal_network_type

NetworkType:Dict[str, int] = {
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
