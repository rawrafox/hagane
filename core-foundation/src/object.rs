use crate::*;

mod ext {
  use crate::*;

  extern {
    pub fn CFGetTypeID(cf: CFTypeRef) -> CFTypeID;
    pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFAutorelease(arg: CFTypeRef) -> CFTypeRef;
    pub fn CFGetRetainCount(cf: CFTypeRef) -> CFIndex;
    pub fn CFEqual(cf1: CFTypeRef, cf2: CFTypeRef) -> Boolean;
    pub fn CFHash(cf: CFTypeRef) -> CFHashCode;
    pub fn CFCopyDescription(cf: CFTypeRef) -> CFStringRef;
    pub fn CFGetAllocator(cf: CFTypeRef) -> CFAllocatorRef;

    pub fn CFShow(obj: CFTypeRef);
  }
}

#[repr(transparent)] pub struct CFTypeRef(crate usize);

unsafe impl Subtype<CFTypeRef> for CFTypeRef {
  unsafe fn upcast(&self) -> CFTypeRef {
    return CFTypeRef(self.0);
  }
}

pub fn CFGetTypeID<T: Subtype<CFTypeRef>>(cf: &T) -> CFTypeID {
  return unsafe { ext::CFGetTypeID(cf.upcast()) };
}

pub unsafe fn CFRetain<T: Subtype<CFTypeRef>>(cf: T) -> T {
  mem::forget(ext::CFRetain(cf.upcast()));

  return cf;
}

pub fn CFRelease<T: Subtype<CFTypeRef>>(cf: T) {
  unsafe { ext::CFRelease(cf.upcast()) };
}

pub unsafe fn CFAutorelease<T: Subtype<CFTypeRef>>(cf: T) -> T {
  mem::forget(ext::CFAutorelease(cf.upcast()));

  return cf;
}

pub fn CFGetRetainCount<T: Subtype<CFTypeRef>>(cf: &T) -> CFIndex {
  return unsafe { ext::CFGetRetainCount(cf.upcast()) };
}

pub fn CFEqual<T1: Subtype<CFTypeRef>, T2: Subtype<CFTypeRef>>(cf1: &T1, cf2: &T2) -> Boolean {
  return unsafe { ext::CFEqual(cf1.upcast(), cf2.upcast()) };
}

pub fn CFHash<T: Subtype<CFTypeRef>>(cf: &T) -> CFHashCode {
  return unsafe { ext::CFHash(cf.upcast()) };
}

pub fn CFCopyDescription<T: Subtype<CFTypeRef>>(cf: &T) -> CFStringRef {
  return unsafe { ext::CFCopyDescription(cf.upcast()) };
}

pub fn CFGetAllocator<T: Subtype<CFTypeRef>>(cf: &T) -> CFAllocatorRef {
  return unsafe { ext::CFGetAllocator(cf.upcast()).retain() };
}

pub fn CFShow<T: Subtype<CFTypeRef>>(cf: &T) {
  unsafe { ext::CFShow(cf.upcast()) };
}

pub trait CFTypeClass : Subtype<CFTypeRef> {
  fn get_type_id(&self) -> CFTypeID {
    return CFGetTypeID(self);
  }

  unsafe fn retain(self) -> Self {
    return CFRetain(self);
  }

  fn release(self) {
    CFRelease(self);
  }

  unsafe fn autorelease(self) -> Self {
    return CFAutorelease(self);
  }

  fn get_retain_count(&self) -> CFIndex {
    return CFGetRetainCount(self);
  }

  fn equal<T: Subtype<CFTypeRef>>(&self, other: &T) -> bool {
    return CFEqual(self, other) == Boolean::TRUE;
  }

  fn hash(&self) -> CFHashCode {
    return CFHash(self);
  }

  fn copy_description(&self) -> CFStringRef {
    return CFCopyDescription(self);
  }

  fn get_allocator(&self) -> CFAllocatorRef {
    return CFGetAllocator(self);
  }

  fn show(&self) {
    CFShow(self);
  }
}

impl<T> CFTypeClass for T where T: Subtype<CFTypeRef> { }

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn it_compares() {
    assert_eq!(CFEqual(kCFNull, kCFAllocatorSystemDefault), Boolean::FALSE);
  }
}

