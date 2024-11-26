import { Component } from '@angular/core';
import { PostsComponent } from './components/posts/posts.component';
import { AlbumsComponent } from './components/albums/albums.component';
import { TodosComponent } from './components/todos/todos.component';
import { UsersComponent } from './components/users/users.component';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css'],
  imports: [PostsComponent, AlbumsComponent, TodosComponent, UsersComponent],
  standalone: true,
})
export class AppComponent {
  title = 'Angular POC';
  selectedComponent: any = null;

  components = [
    { name: 'Posts', component: PostsComponent },
    { name: 'Albums', component: AlbumsComponent },
    { name: 'Todos', component: TodosComponent },
    { name: 'Users', component: UsersComponent }
  ];

  loadComponent(component: any) {
    this.selectedComponent = component;
  }

  constructor() {}

}