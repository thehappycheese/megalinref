

from typing import Any, List, Dict, Tuple, TypedDict, Literal


Cwy:Dict[str, int]


NetworkType:Dict[str, int]

FeatureCwy = Literal['Left', 'Single', 'Right']
FeatureNetworkType = Literal['Local Road', 'Crossover', 'Proposed Road', 'Miscellaneous Road', 'Main Roads Controlled Path', 'State Road']

class FeatureRowAttributes(TypedDict):
    ROAD: str
    CWY: FeatureCwy
    NETWORK_TYPE: FeatureNetworkType
    START_SLK: float
    END_SLK: float
    START_TRUE_DIST: float
    END_TRUE_DIST: float

class Result_road_slk_from_coordinate(TypedDict):
    feature:FeatureRowAttributes
    slk: float
    true: float
    distance_metres:float


class Lookup:
    """Do we get to add docstrings like this?"""
    
    @staticmethod
    def from_dict(road_network_data:Dict[str, Any]) -> Lookup:
        """
        Lookup is a class that can be used to lookup Road Number and SLK from lat/lon or vice versa.

        Args:
            road_network_data: The road network data as a dict.
            Obtain using `megalinref.download_fresh_data_as_json()`
            Should be parsed GeoJSON `"FeatureCollection"` format containing only `"LineString"` features.
        """
        ...

    @staticmethod
    def from_binary(binary:bytes) -> Lookup:
        """Creates a new SLKLookup object from a bytes object.
        See also `SLKLookup.to_binary()`.
        
        Caching the data in binary is much faster than loading the data from JSON text; 
         - Loading json using `SLKLookup(json.load(f))` takes about 8 seconds.
         - Loading binary using `SLKLookup.from_binary(f.read())` takes about 0.8 seconds.

        Example:
        ```python
        import megalinref as mlr

        road_network_data = mlr.download_fresh_data_as_json()
        slk_lookup = mlr.Lookup.from_dict(road_network_data)
        del road_network_data

        # save to disk in binary format
        bincode = slk_lookup.to_binary()
        with open("./temporary_test_data/binary_cache.bin", "wb") as f:
            f.write(bincode)

        # load from disk in binary format
        with open("./temporary_test_data/binary_cache.bin", "rb") as f:
            bincode = f.read()
        slk_lookup_reloaded = mlr.Lookup.from_binary(bincode)

        ```        
        """
        ...

    
    def to_binary(self) -> bytes:
        """Converts the road network data stored in this  into bytes so that it can be 
        saved to disk in a compact format.
        The data can then be restored then restore it with `Lookup.from_binary().`
        
        Caching the data in binary is much faster than loading the data from JSON text; 
         - Loading json using `Lookup(json.load(f))` takes about 8 seconds.
         - Loading binary using `Lookup.from_binary(f.read())` takes about 0.8 seconds.

        Example:
        ```python
        import megalinref as mlr

        road_network_data = mlr.download_fresh_data_as_json()
        slk_lookup = mlr.Lookup.from_dict(road_network_data)
        del road_network_data

        # save to disk in binary format
        bincode = slk_lookup.to_binary()
        with open("./temporary_test_data/binary_cache.bin", "wb") as f:
            f.write(bincode)

        # load from disk in binary format
        with open("./temporary_test_data/binary_cache.bin", "rb") as f:
            bincode = f.read()
        slk_lookup_reloaded = mlr.Lookup.from_binary(bincode)

        ```        
        """
        ...


    def road_slk_from_coordinate(
        self,
        lat:float,
        lon:float,
        carriageways:int,
        network_types:int,
        roads:List[str]
    ) -> Result_road_slk_from_coordinate:
        """
        Returns the Road Number and SLK for for the road closest to the given lat/lon coordinate..
        
        Input coordinates are in degrees, in the WSG84 Geographic Coordinate System.
        
        The `cwy` parameter is a bitflag, and can accept binary combinations of the megalinref.Cwy dictionary.
        For example, to find only Left and Single carriageway roads, use `cwy=megalinref.Cwy["Left"] | megalinref.Cwy["Single"]`.

        Args:
            lat:          The latitude of the coordinate.
            lon:          The longitude of the coordinate.
            cwy:          The carriageway to search for. See docs for `megalinref.Cwy`.
            network_type: The network type to search for. See docs for `megalinref.NetworkType`.
            roads:        A list of road numbers eg ['H001', 'H002']. Use a blank list if no filtering is desired. Long lists will be bad for performance. Try use only one or two road numbers if possible.

        Returns:
            A dict containing the Road deatails, the slk, true, and the distance from the road to the input lat/lon coordinate.

        # result
        ```text
            {
                "feature":{
                    "ROAD":         "H016",
                    "CWY":          "Left",
                    "NETWORK_TYPE": "State Road",
                    "START_SLK":    8,
                    "END_SLK":      11,
                    "START_TRUE":   8,
                    "END_TRUE":     11
                },
                "slk": 10,
                "true":10,
                "distance_metres":0,
            }
        ```
        """
        ...


    def coordinate_from_road_slk(
        self,
        road:str,
        slk:float,
        carriagways:int
    ) -> List[List[Tuple[float, float]]]:
        """
        Returns the lat/lon coordinates for the given Road Number and SLK.

        Args:
            road: The Road Number to search for.
            slk: The SLK to search for.
            carriageways: The carriageways to search for. See `megalinref.Cwy`.
        
        Returns:
            A list for each carriageway containing a list of lat/lon coordinates.
        """
        ...