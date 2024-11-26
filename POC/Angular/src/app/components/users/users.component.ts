import { Component, OnInit } from '@angular/core';
import { JsonPlaceholderService } from '../../services/jsonplaceholder.service';

@Component({
  selector: 'app-users',
  standalone: true,
  imports: [],
  templateUrl: './users.component.html',
  styleUrl: './users.component.css'
})
export class UsersComponent implements OnInit {
  users: any[] = [];

  constructor(private jsonPlaceholderService: JsonPlaceholderService) {}

  ngOnInit(): void {
    this.jsonPlaceholderService.getUsers().subscribe((data) => {
      this.users = data;
    });
  }
}
