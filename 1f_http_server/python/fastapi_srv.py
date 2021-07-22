from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles

app = FastAPI()

app.mount("/static", StaticFiles(directory="../static"), name="static")


@app.get("/")
@app.get("/hello/{name}")
async def root(name: str = "World"):
    return {"message": f"Hello, {name}!"}


@app.get("/items/{item_id}")
async def read_item(item_id: int):
    return {"item_id": item_id}
