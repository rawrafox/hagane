use std;

use objc::declare::ClassDecl;
use objc::runtime::{Object, Sel};

use super::RSMViewID;

use hagane_objc::*;
use foundation::*;
use core_graphics::*;
use cocoa::*;
use metal::*;
use metal_kit::*;

pub trait RSMRenderer {
  fn initialize(&mut self, view: RSMViewID);
  fn draw(&mut self, view: RSMViewID);
}

impl RSMViewID {
  pub fn from_renderer<T: MTLDevice + 'static>(renderer: Box<RSMRenderer>, frame: CGRect, device: &T) -> RSMViewID {
    load_classes();

    let mut renderer = Box::new(renderer);

    let view = RSMViewID::alloc().init_with_frame_device(frame, device);

    (**renderer).initialize(view.clone());

    unsafe {
      view.as_object().set_ivar("renderer", Box::into_raw(renderer) as *mut i8);
    }

    return view;
  }
}

fn load_view() {
  let mut view = ClassDecl::new("RSMView", MTKViewID::class()).unwrap();

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

  unsafe {
    view.add_method(sel!(dealloc), dealloc as extern fn(&mut Object, Sel));
    view.add_method(sel!(drawRect:), draw_rect as extern fn(&mut Object, Sel, usize));
  }

  view.add_ivar::<*mut i8>("renderer");
  view.register();
}

fn load_window_delegate() {
  let mut window_delegate = ClassDecl::new("RSMWindowDelegate", NSObjectID::class()).unwrap();

  extern fn window_will_close(this: &mut Object, _cmd: Sel, _notification: usize) {
    NSApplicationID::shared_application().terminate(&NSObjectID::from_object(this));
  }

  unsafe {
    window_delegate.add_method(sel!(windowWillClose:), window_will_close as extern fn(&mut Object, Sel, usize));
  }

  window_delegate.register();
}

static START: std::sync::Once = std::sync::ONCE_INIT;

pub fn load_classes() {
  START.call_once(|| {
    load_view();
    load_window_delegate();
  });
}
