import asyncio
import megalinref

from megalinref import megalinref

def test_geo():
    result = megalinref.how_to_import()
    

async def fetch_data():
    await megalinref.mlr.fetch_data("https://mrgis.mainroads.wa.gov.au/arcgis/rest/services/OpenData/RoadAssets_DataPortal/MapServer/17/query?where=1%3D1&outFields=ROAD,START_SLK,END_SLK,CWY&outSR=4326&f=json")



def test_fetch_data():
    asyncio.run(fetch_data())
