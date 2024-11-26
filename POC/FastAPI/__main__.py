from fastapi import FastAPI, Depends, HTTPException, Request
from sqlmodel import Session
from uvicorn import Server, Config
from contextlib import asynccontextmanager
from database import init_db, get_session
from models import Todo
from crud import create_todo, get_todos, get_todo, update_todo, delete_todo
from starlette.responses import RedirectResponse

from requests_oauthlib import OAuth2Session
import os
from dotenv import load_dotenv
from fastapi.responses import HTMLResponse

@asynccontextmanager
async def lifespan(app: FastAPI):
    print("Starting up")
    init_db()
    yield
    print("Shutting down")

app = FastAPI(lifespan=lifespan)

load_dotenv()
os.environ['OAUTHLIB_INSECURE_TRANSPORT'] = '1'

client_id = os.getenv('CLIENT_ID')
client_secret = os.getenv('CLIENT_SECRET')
authorization_base_url = 'https://accounts.google.com/o/oauth2/auth'
token_url = 'https://oauth2.googleapis.com/token'
redirect_uri = 'http://localhost:5000/callback'

scopes = ['openid', 'https://www.googleapis.com/auth/userinfo.profile', 'https://www.googleapis.com/auth/userinfo.email']

oauth = OAuth2Session(client_id, redirect_uri=redirect_uri, scope=scopes)

@app.post("/todos/", response_model=Todo)
def create_new_todo(todo: Todo, session: Session = Depends(get_session)):
    return create_todo(session, todo)

@app.get("/todos/", response_model=list[Todo])
def read_todos(session: Session = Depends(get_session)):
    return get_todos(session)

@app.get("/todos/{todo_id}", response_model=Todo)
def read_todo(todo_id: int, session: Session = Depends(get_session)):
    todo = get_todo(session, todo_id)
    if not todo:
        raise HTTPException(status_code=404, detail="Todo not found")
    return todo

@app.put("/todos/{todo_id}", response_model=Todo)
def update_existing_todo(todo_id: int, todo: Todo, session: Session = Depends(get_session)):
    todo = get_todo(session, todo_id)
    if not todo:
        raise HTTPException(status_code=404, detail="Todo not found")
    todo.title = todo.title
    todo.description = todo.description
    return update_todo(session, todo)

@app.delete("/todos/{todo_id}", response_model=Todo)
def delete_existing_todo(todo_id: int, session: Session = Depends(get_session)):
    todo = get_todo(session, todo_id)
    if not todo:
        raise HTTPException(status_code=404, detail="Todo not found")
    return delete_todo(session, todo)

@app.get("/login")
async def login():
    authorization_url, state = oauth.authorization_url(authorization_base_url, access_type="offline", prompt="consent")
    return authorization_url

@app.get("/callback")
async def callback(request: Request):
    authorization_response = str(request.url)
    token = oauth.fetch_token(token_url, authorization_response=authorization_response, client_secret=client_secret)
    response = oauth.get('https://www.googleapis.com/oauth2/v1/userinfo?alt=json')
    if response.status_code == 200:
        user_info = response.json()
        return HTMLResponse(content=f"Token récupéré avec succès !<br>Info utilisateur : {user_info}<br>Token : {token}")
    else:
        return f"Erreur lors de la récupération des informations utilisateur: {response.status_code}"

if __name__ == "__main__":
    config = Config(app=app, host="127.0.0.1", port=5000)
    server = Server(config)
    try:
        server.run()
    except KeyboardInterrupt:
        pass
