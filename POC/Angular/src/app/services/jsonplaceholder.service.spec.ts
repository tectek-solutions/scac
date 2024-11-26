import { TestBed } from '@angular/core/testing';

import { JsonPlaceholderService } from './jsonplaceholder.service';

describe('JsonplaceholderService', () => {
  let service: JsonPlaceholderService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(JsonPlaceholderService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
