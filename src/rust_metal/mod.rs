use std;
use objc;

use objc::declare::ClassDecl;
use objc::runtime::{Object, Sel};

use super::{ObjectiveC, CGRect, NSApplication, NSApplicationID, NSObject, NSObjectID, NSView, NSWindowDelegate, MTKView, MTKViewID, MTLDevice};

pub trait RSMRenderer {
  fn initialize(&mut self, view: RSMViewID);
  fn draw(&mut self, view: RSMViewID);
}

pub trait RSMView : MTKView {
  forward!(initialize_object, sel!(initializeObject), () -> ());
}

id!(RSMViewID, RSMView, "RSMView");

impl RSMViewID {
  pub fn load_class() {
    let mut klass = ClassDecl::new("RSMView", MTKViewID::class()).unwrap();

    klass.add_ivar::<*mut i8>("renderer");

    extern fn dealloc(this: &mut Object, _cmd: Sel) {
      unsafe {
        let renderer_ptr: &*mut i8 = this.get_ivar("renderer");

        Box::from_raw(std::mem::transmute::<*mut i8, *mut Box<RSMRenderer>>(*renderer_ptr));
      }
    }

    extern fn draw_rect(this: &mut Object, _cmd: Sel, _rect: usize) {
      unsafe {
        let view = RSMViewID::from_object(this);
        let renderer_ptr: &*mut i8 = this.get_ivar("renderer");
        let mut renderer = Box::from_raw(std::mem::transmute::<*mut i8, *mut Box<RSMRenderer>>(*renderer_ptr));

        (**renderer).draw(view.clone());

        Box::into_raw(renderer);
        
        std::mem::forget(view);
      }
    }

    extern fn window_will_close(this: &mut Object, _cmd: Sel, _notification: usize) {
      NSApplicationID::shared_application().terminate(NSObjectID::from_object(this));
    }

    unsafe {
      klass.add_method(sel!(dealloc), dealloc as extern fn(&mut Object, Sel));
      klass.add_method(sel!(drawRect:), draw_rect as extern fn(&mut Object, Sel, usize));
      klass.add_method(sel!(windowWillClose:), window_will_close as extern fn(&mut Object, Sel, usize));
    }

    klass.register();
  }

  pub fn from_renderer<T: MTLDevice + 'static>(renderer: Box<RSMRenderer>, frame: CGRect, device: T) -> RSMViewID {
    let mut renderer = Box::new(renderer);

    let view = RSMViewID::alloc().init_with_frame_device(frame, device);

    (**renderer).initialize(view.clone());

    unsafe {
      view.as_object().set_ivar("renderer", Box::into_raw(renderer) as *mut i8);
    }

    return view;
  }
}

impl NSObject for RSMViewID {}
impl NSView for RSMViewID {}
impl MTKView for RSMViewID {}
impl NSWindowDelegate for RSMViewID {}
