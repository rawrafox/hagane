pub unsafe trait Subtype<T> : Sized {
  unsafe fn upcast(&self) -> T;
}
