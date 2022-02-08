

Cwy:dict[str, int]


NetworkType:dict[str, int]


class SLKLookup:
    """Do we get to add docstrings like this?"""
    
    def __init__(self, road_network_data:dict):
        """
        SLKLookup is a class that can be used to lookup Road Number and SLK from lat/lon
        Args:
            road_network_data: The road network data as a dict. Obtain using `megalinref.download_fresh_data_as_json()`
        """
        ...

    @staticmethod
    def from_binary(binary:bytes) -> SLKLookup:
        """Creates a new SLKLookup object from a bytes object.
        See also `SLKLookup.to_binary()`.
        
        Caching the data in binary is much faster than loading the data from JSON text; 
         - Loading json using `SLKLookup(json.load(f))` takes about 8 seconds.
         - Loading binary using `SLKLookup.from_binary(f.read())` takes about 0.8 seconds.

        Example:
        ```python
        import megalinref as mlr

        road_network_data = mlr.download_fresh_data_as_json()
        slk_lookup = mlr.SLKLookup(road_network_data)
        del road_network_data

        # save to disk in binary format
        bincode = slk_lookup.to_binary()
        with open("./temporary_test_data/binary_cache.bin", "wb") as f:
            f.write(bincode)

        # load from disk in binary format
        with open("./temporary_test_data/binary_cache.bin", "rb") as f:
            bincode = f.read()
        slk_lookup_reloaded = mlr.SLKLookup.from_binary(bincode)

        ```        
        """
        ...

    
    def to_binary() -> bytes:
        """Converts the road network data stored in this  into bytes so that it can be 
        saved to disk in a compact format.
        The data can then be restored then restore it with `SLKLookup.from_binary().`
        
        Caching the data in binary is much faster than loading the data from JSON text; 
         - Loading json using `SLKLookup(json.load(f))` takes about 8 seconds.
         - Loading binary using `SLKLookup.from_binary(f.read())` takes about 0.8 seconds.

        Example:
        ```python
        import megalinref as mlr

        road_network_data = mlr.download_fresh_data_as_json()
        slk_lookup = mlr.SLKLookup(road_network_data)
        del road_network_data

        # save to disk in binary format
        bincode = slk_lookup.to_binary()
        with open("./temporary_test_data/binary_cache.bin", "wb") as f:
            f.write(bincode)

        # load from disk in binary format
        with open("./temporary_test_data/binary_cache.bin", "rb") as f:
            bincode = f.read()
        slk_lookup_reloaded = mlr.SLKLookup.from_binary(bincode)

        ```        
        """
        ...

        def lookup() -> dict: