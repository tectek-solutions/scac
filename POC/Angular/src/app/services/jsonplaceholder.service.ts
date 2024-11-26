import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class JsonPlaceholderService {
  private apiUrl = 'https://jsonplaceholder.typicode.com';

  constructor(private http: HttpClient) {}

  getPosts(): Observable<any> {
    return this.http.get(`${this.apiUrl}/posts`);
  }

  editPost(id: number, postData: { title: string; body: string; userId: number }): Observable<any> {
    return this.http.put<any>(`${this.apiUrl}/posts/${id}`, postData);
  }

  deletePost(id: number): Observable<any> {
    return this.http.delete(`${this.apiUrl}/posts/${id}`);
  }

  addPost(postData: { title: string; body: string; userId: number }): Observable<any> {
    return this.http.post<any>(`${this.apiUrl}/posts`, postData);
  }

  getAlbums(): Observable<any> {
    return this.http.get(`${this.apiUrl}/albums`);
  }

  getTodos(): Observable<any> {
    return this.http.get(`${this.apiUrl}/todos`);
  }

  getUsers(): Observable<any> {
    return this.http.get(`${this.apiUrl}/users`);
  }
}