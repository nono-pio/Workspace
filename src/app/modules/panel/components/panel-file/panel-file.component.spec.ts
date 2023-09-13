import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PanelFileComponent } from './panel-file.component';

describe('PanelFileComponent', () => {
  let component: PanelFileComponent;
  let fixture: ComponentFixture<PanelFileComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ PanelFileComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(PanelFileComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
