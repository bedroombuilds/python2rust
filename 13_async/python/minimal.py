import asyncio

async def a():
    print('Hello ...')
    await asyncio.sleep(3)
    print("waited")
    return 1

async def b():
    print('... World!')
    return 2

async def main():
    await a()
    await b()
    print("serial\n")

    result = await asyncio.gather(a(), b())
    print("joined\n")
    assert result == [1, 2]

    done, pending = await asyncio.wait(
        [asyncio.create_task(a()), asyncio.create_task(b())], return_when=asyncio.FIRST_COMPLETED)
    for p in pending:
        p.cancel()
    print("done {}\n".format(done.pop().result()))

    futures = [b(), a(), b()]
    result = await asyncio.gather(*futures)
    print("joined vector\n")

if __name__ == "__main__":
    # Python 3.7+
    asyncio.run(main())
