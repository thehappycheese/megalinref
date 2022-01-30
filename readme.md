# megalinref - Mega-fast Linear Referencing

A Python module built in rust for the mega-speed you deserve.

This library is in early stages of development, but when completed, it will:

1. convert coordinates road number, carriageway and SLKs,
1. convert SLK observations converted into coordinates
   - Previously I implemented a pure Rust server for this purpose; see <https://github.com/thehappycheese/nicklinref_rust>

- [x] Got rust to build, and python to import
- [x] Got data downloaded in python
- [x] Got GeoJSON passed into rust datastructure
- [ ] Insert geometry into some sort of RTree structure
- [ ] Get reverse spatial lookup working
  - [ ] lat/lon to Road/Cwy/SLK, one point at a time
  - [ ] lat/lon to Road/Cwy/SLK, from numpy array of coordinates
- [ ] Get forward spatial lookup working
  - [ ] Road/Cwy/SLK to lat/lon, one point at a time
  - [ ] Road/Cwy/SLK to numpy array of coordinates
- [ ] Publish on Conda-Forge with windows binaries
  - [ ] Publish anylinux binaries (if it ever seems like enough people might use the package)

## Usage

The interface is currently awful and messy, but it will look something like this:

```python
import megalinref
from megalinref.megalinref import SLKLookup

xx = megalinref.data_handling._download_fresh_data_as_json(chunk_limit=None))

slk_lookup = SLKLookup(xx)

(road, cwy, slk) = slk_lookup.lookup(lat=-31.956598, lon=115.8773259)

```

## Why write a Python Library in Rust?

Well I already have some previous works

- a python library to convert lat/lon to SLK <https://github.com/thehappycheese/linrefreverse>
  - It is slow and uses loads of ram.
  - It depends on `geopandas` `SpatialIndex.nearest()` which in turn depends on `pygeos` or `rtree`

- A pure Rust web service that converts Road Name & SLK into lat/lon <https://github.com/thehappycheese/nicklinref_rust>
  - This is super fast, but has the drawback of generating lots of network traffic when not running at localhost, and just generally being difficult to deploy. Also, making web requests can sometimes get in the way of the task at hand.

This new package brings all the functionality to into Python with the power of an import statement, without sacrificing any speed to `geopandas` clunky backend, or having to deal with web requests.

There are downsides: Building this library using Rust means there will be trouble distributing binaries. Most likely this will only work on x64 Windows for the forseeable future, and distributing the binaries will be difficult. Perhaps it can be deployed to conda-forge.