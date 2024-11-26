from sqlmodel import Session, select
from models import Todo

def create_todo(session: Session, todo: Todo) -> Todo:
    session.add(todo)
    session.commit()
    session.refresh(todo)
    return todo

def get_todos(session: Session):
    return session.exec(select(Todo)).all()

def get_todo(session: Session, todo_id: int):
    return session.get(Todo, todo_id)

def update_todo(session: Session, todo: Todo):
    session.add(todo)
    session.commit()
    session.refresh(todo)
    return todo

def delete_todo(session: Session, todo: Todo):
    session.delete(todo)
    session.commit()
    return todo
