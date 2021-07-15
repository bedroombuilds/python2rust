import asyncio
import websockets
import sys


async def read_stdin_line():
    loop = asyncio.get_event_loop()
    return await loop.run_in_executor(None, sys.stdin.readline)


async def stdin_to_ws(websocket):
    while True:
        line = await read_stdin_line()
        await websocket.send(line.strip())
        if line.strip() == "/exit":
            break


async def ws_next(websocket):
    try:
        return await websocket.recv()
    except:
        return None


async def main():
    uri = "ws://localhost:8000/chat"
    async with websockets.connect(uri) as websocket:
        print("connected successfully")
        while True:
            stdin = asyncio.create_task(stdin_to_ws(websocket))
            ws = asyncio.create_task(ws_next(websocket))
            done, _ = await asyncio.wait([stdin, ws], return_when=asyncio.FIRST_COMPLETED)
            msg = done.pop()
            if msg.result():
                print(msg.result())
            else:
                break


if __name__ == "__main__":
    asyncio.get_event_loop().run_until_complete(main())
