import { Component, OnInit } from '@angular/core';
import { JsonPlaceholderService } from '../../services/jsonplaceholder.service';

@Component({
  selector: 'app-albums',
  standalone: true,
  imports: [],
  templateUrl: './albums.component.html',
  styleUrl: './albums.component.css'
})
export class AlbumsComponent implements OnInit {
  albums: any[] = [];
  newAlbum: any = { title: '' };
  
  constructor(private jsonPlaceholderService: JsonPlaceholderService) {}

  ngOnInit(): void {
    this.jsonPlaceholderService.getAlbums().subscribe((data) => {
      this.albums = data;
    });
  }
}
