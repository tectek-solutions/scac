import { Component, OnInit } from '@angular/core';
import { JsonPlaceholderService } from '../../services/jsonplaceholder.service';

@Component({
  selector: 'app-posts',
  templateUrl: './posts.component.html',
  styleUrls: ['./posts.component.css'],
  standalone: true,
})
export class PostsComponent implements OnInit {
  posts: any[] = [];
  nextId: number = 1;

  constructor(private jsonPlaceholderService: JsonPlaceholderService) {}

  ngOnInit(): void {
    this.jsonPlaceholderService.getPosts().subscribe((data) => {
      this.posts = data;
    });
  }

  deletePost(id: number): void {
    console.log(id);
    this.jsonPlaceholderService.deletePost(id).subscribe(() => {
      this.posts = this.posts.filter(post => post.id !== id);
    });
  }

  addPost() {
    const newPost = { title: 'New Post', body: 'This is a new post.', userId: 1, id: this.nextId };
    this.jsonPlaceholderService.addPost(newPost).subscribe(() => {
      this.posts.unshift(newPost);
      this.nextId++;
    });
  }

  editPost(id: number) {
    const post = this.posts.find(post => post.id === id);
    const updatedPost = { title: 'Updated Post', body: 'This post has been updated.', userId: 1 };
    this.jsonPlaceholderService.editPost(id, updatedPost).subscribe(() => {
      post.title = updatedPost.title;
      post.body = updatedPost.body;
    });
  }
}