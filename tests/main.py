import asyncio
import megalinref


result = megalinref.test_geo()
print(f"success probably? {result}")

async def main():
    result = await megalinref.fetch_data("https://mrgis.mainroads.wa.gov.au/arcgis/rest/services/OpenData/RoadAssets_DataPortal/MapServer/17/query?where=1%3D1&outFields=ROAD,START_SLK,END_SLK,CWY&outSR=4326&f=json")
    print(f"success probably? {result}")

asyncio.run(main())