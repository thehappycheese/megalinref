# megalinref - Mega-fast Linear Referencing <!-- omit in toc -->

Python module for converting between lat / lon and road / chainage.

Uses Rust binaries in the backend for the mega-speed you deserve :)

1. Convert coordinates road number, carriageway and SLKs,
2. Convert SLK observations converted into coordinates
   - Previously I implemented a pure Rust server for this purpose; see <https://github.com/thehappycheese/nicklinref_rust>

- [1. Development Progress](#1-development-progress)
- [2. Installation](#2-installation)
- [3. Usage / Examples](#3-usage--examples)
- [4. Why write a Python Library in Rust?](#4-why-write-a-python-library-in-rust)
- [5. Setup for Development](#5-setup-for-development)
  - [5.1. Initial Setup](#51-initial-setup)
  - [5.2. Build using `maturin`](#52-build-using-maturin)
- [6. Previous Projects](#6-previous-projects)
  - [6.1. linrefreverse](#61-linrefreverse)
  - [6.2. nicklinref_rust](#62-nicklinref_rust)

## 1. Development Progress

This library is in early stages of development. Road map below:

- [x] Get rust to build, and python to import
- [x] Get data downloaded in python
- [x] Get GeoJSON passed into rust data structure
  - [x] Get rust to serialize and compress its internal data structure
  - [x] Confirm binary caching is working
- [x] Get reverse spatial lookup working
  - [ ] Insert geometry into some sort of RTree structure for high speed
    - [ ] Make the RTree structure it actually perform as fast as I expected :(
      this is turning out to be pretty hard.
  - [x] Gave up on the RTree structure, brute force parallel with `rayon` works
    better :/
    - Available libraries in the ecosystem (I tried `rstar`) seem ill equipped
      to deal with ~ 250 MB of linestrings. The memory usage just explodes
      beyond reasonable limits (5GB+) and I was not able to insert the entire
      road network. I think i may have been having some other kind of technical
      problem at the time... perhaps i can try again.
    - I don't really want to try the rust `GEOS` bindings. I doubt I can do any
      better than the already existing `shapely` library if I must resort to
      that.
  - [x] map lat/lon to Road/Cwy/SLK, one point at a time
  - [ ] map lat/lon to Road/Cwy/SLK, from numpy array of coordinates
  - [x] implement filters for state road / cwy etc
  - [ ] implement filters for specific road (so we don't accidentally match a
    side road when a coordinate happens at an intersection)
- [x] Get forward spatial lookup working
  - [x] Build hash table based on road number
  - [x] map Road/Cwy/SLK to lat/lon, one point at a time
  - [ ] map Road/Cwy/SLK to numpy array of coordinates
- [ ] Publish on Conda-Forge with windows binaries
  - [ ] Publish anylinux binaries (if it ever seems like enough people might use
    the package, or if I need to use it on some cloud platform)

## 2. Installation

See the [releases](https://github.com/thehappycheese/megalinref/releases) page
for instructions on how to install each release.

## 3. Usage / Examples

The following is subject to change in future releases:

```python
from megalinref import (
    Lookup,
    Cwy,
    NetworkType,
    download_fresh_data_as_json
)

road_network = download_fresh_data_as_json()
slk_lookup = Lookup.from_dict(road_network)

# the download step above takes a while,
# lets cache the lookup object so we don't
# have to repeat the download next time:
with open("cached_lookup.bin", "wb") as bin_cache:
    bin_cache.write(slk_lookup.to_binary())

# For demonstration purposes lets reload 
# a new copy of `lookup` from the cache
with open("cached_lookup.bin", "rb") as bin_cache:
    slk_lookup = Lookup.from_binary(
        bin_cache.read()
    )

# lets lookup a road/slk based on a lat/lon
result = slk_lookup.road_slk_from_coordinate(
    lat           = -31.89006203575722,
    lon           = 115.80183730752809,
    carriageways  = Cwy["L"] | Cwy["R"],
    network_types = NetworkType["State Road"] | NetworkType["Local Road"] 
)
result["slk"]             = round(result["slk"],             3)
result["true"]            = round(result["true"],            3)
result["distance_metres"] = round(result["distance_metres"], 3)
assert result == {
    'feature': {
        'ROAD': 'H016',
        'CWY': 'Left',
        'START_SLK': 9.84,
        'END_SLK': 10.68,
        'START_TRUE_DIST': 9.84,
        'END_TRUE_DIST': 10.68,
        'NETWORK_TYPE': 'State Road'
    },
    'slk': 10.000,
    'true': 10.000,
    'distance_metres': 0.000
}

# now lets lookup a lat/lon based on a road/slk
result = slk_lookup.coordinate_from_road_slk(
    road         = "H016",
    slk          =8.5,
    carriageways =Cwy["L"]
)
assert result == [[(115.81402235326775, -31.897493888518945)]]
# the result type will be improved in future; currently you get a list of up to
# three lists (corresponding with Left, Single and Right Carriageway), each
# containing one or more coordinates as tuples.
```

## 4. Why write a Python Library in Rust?

`megalinref` brings all the functionality of my [previous projects](#6-previous-projects) to into Python

- by the power of a single `import`,
- without sacrificing any speed to `geopandas`,
- with no clunky backends
- or having to deal with web requests.

There are downsides: Building this library using Rust means there will be trouble distributing binaries. Most likely this will only work on x64 Windows for the forseeable future, and distributing the binaries will be difficult. Perhaps it can be deployed to conda-forge.

## 5. Setup for Development

### 5.1. Initial Setup

These instructions assume you already have python 3.9+, rust and cargo installed.

The following commands will create and activate a virtual environment in the folder `.env/`

```console
python -m pip install --user --upgrade pip
python -m pip install --user --upgrade virtualenv
python -m venv .env
```

Activate the new environment on windows;

```powershell
.\.env\Scripts\activate
```

Or on linux;

```bash
. .env/bin/activate
```

Next we need a build tool called maturin (see documentation [PyO3/maturin](https://github.com/PyO3/maturin)) installed into the virtual environment.

```console
pip install maturin
```

To make jupyter notebooks work, and to run tests also do:

```console
pip install ipykernel
pip install pytest
```

### 5.2. Build using `maturin`

To build on Amazon Linux Container:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo yum update
sudo yum groupinstall "Development Tools"
```

For windows follow the guidance to install Rust on the Rust website.

build for testing using

```console
maturin develop --release
```

or build wheel using

```console
maturin build --release --interpreter python
```

> NOTE: I had to add the `--interpreter python` flag since apparently `maturin` looks for the `py` launcher on the `PATH` and in my case there is no such launcher installed.

## 6. Previous Projects

### 6.1. linrefreverse

<https://github.com/thehappycheese/linrefreverse>

- a python library to convert lat/lon to SLK
  - It is slow and uses loads of ram.
  - It depends on `geopandas` `SpatialIndex.nearest()` which in turn depends on `pygeos` or `rtree`.
    - `geopandas` itself seems to hog tonnes of ram somehow
    - I think i tested and found `rtree` only handles point objects, not lines?? I don't know why it wasn't working
    - pygeos depends on the legendary `GEOS` C++ package. Pretty sure this handled linestrings and did the job nice and fast. But `pygeos` is an optional dependency and you have to set flags in `geopandas` to enable it as the default backend. Don't like.

### 6.2. nicklinref_rust

<https://github.com/thehappycheese/nicklinref_rust>

- A pure Rust web service that converts Road Name & SLK into lat/lon
  - This is super fast, but has the drawback of generating lots of network traffic when not running at localhost, and just generally being difficult to deploy. Also, making web requests can sometimes get in the way of the task at hand.
