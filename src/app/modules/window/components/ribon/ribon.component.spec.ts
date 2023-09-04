import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RibonComponent } from './ribon.component';

describe('RibonComponent', () => {
  let component: RibonComponent;
  let fixture: ComponentFixture<RibonComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ RibonComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(RibonComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
