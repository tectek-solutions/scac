import { Component, OnInit } from '@angular/core';
import { JsonPlaceholderService } from '../../services/jsonplaceholder.service';

@Component({
  selector: 'app-todos',
  standalone: true,
  imports: [],
  templateUrl: './todos.component.html',
  styleUrl: './todos.component.css'
})
export class TodosComponent implements OnInit {
  todos: any[] = [];

  constructor(private jsonPlaceholderService: JsonPlaceholderService) {}

  ngOnInit(): void {
    this.jsonPlaceholderService.getTodos().subscribe((data) => {
      this.todos = data;
    });
  }
}
