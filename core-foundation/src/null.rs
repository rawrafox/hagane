use crate::*;

mod ext {
  use crate::*;

  extern {
    pub fn CFNullGetTypeID() -> CFTypeID;

    pub static kCFNull: CFNullRef;
  }
}

#[repr(transparent)] pub struct CFNullRef(usize);

unsafe impl Subtype<CFTypeRef> for CFNullRef {
  unsafe fn upcast(&self) -> CFTypeRef {
    return CFTypeRef(self.0);
  }
}

unsafe impl Subtype<CFNullRef> for CFNullRef {
  unsafe fn upcast(&self) -> CFNullRef {
    return CFNullRef(self.0);
  }
}


pub fn CFNullGetTypeID() -> CFTypeID {
  return unsafe { ext::CFNullGetTypeID() };
}

pub static kCFNull: &'static CFNullRef = unsafe { &ext::kCFNull };

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn it_nulls() {
    assert_eq!(CFNullGetTypeID(), CFGetTypeID(kCFNull));
    assert_eq!(CFNullGetTypeID(), kCFNull.get_type_id());
  }
}
