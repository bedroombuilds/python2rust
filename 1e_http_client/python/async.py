import aiohttp
import asyncio
import time

from person import URL, summary


async def fetch_person(session, url):
    async with session.get(url) as resp:
        json = await resp.json()
        return json


async def main():
    async with aiohttp.ClientSession() as session:
        tasks = []
        for _ in range(50):
            tasks.append(asyncio.ensure_future(fetch_person(session, URL)))

        all_data = await asyncio.gather(*tasks)
        for data in all_data:
            print(summary(data))


if __name__ == "__main__":
    start_time = time.time()
    asyncio.run(main())
    print("--- %s seconds ---" % (time.time() - start_time))
