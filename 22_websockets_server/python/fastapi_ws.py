from atomic import AtomicCounter

from fastapi import FastAPI, WebSocket, WebSocketDisconnect
from fastapi.responses import HTMLResponse

app = FastAPI()


class ConnectionManager:
    def __init__(self):
        self.user_counter = AtomicCounter()
        self.active_connections = dict()

    async def connect(self, websocket: WebSocket):
        await websocket.accept()
        client_id = self.user_counter.inc()
        self.active_connections[client_id] = websocket

    def disconnect(self, websocket: WebSocket):
        k = self.client_id(websocket)
        if k:
            del self.active_connections[k]

    def client_id(self, websocket: WebSocket):
        return next((uid for uid, ws in self.active_connections.items() if ws == websocket), None)

    async def broadcast(self, message: str, websocket):
        for connection in filter(lambda c: c != websocket, self.active_connections.values()):
            await connection.send_text(message)


manager = ConnectionManager()


@app.get("/")
async def get():
    with open('../static/index.html') as f:
        html = f.read()
    return HTMLResponse(html)


@app.websocket("/chat")
async def websocket_endpoint(websocket: WebSocket):
    await manager.connect(websocket)
    try:
        while True:
            data = await websocket.receive_text()
            await manager.broadcast(f"<User#{manager.client_id(websocket)}>: {data}", websocket)
    except WebSocketDisconnect:
        user_id = manager.client_id(websocket)
        manager.disconnect(websocket)
        await manager.broadcast(f"User#{user_id} left the chat", None)
