import { ComponentFixture, TestBed } from '@angular/core/testing';

import { VerticalSplitComponent } from './vertical-split.component';

describe('VerticalSplitComponent', () => {
  let component: VerticalSplitComponent;
  let fixture: ComponentFixture<VerticalSplitComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ VerticalSplitComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(VerticalSplitComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
