pub trait Observable<T> {
    fn observe(&self) -> T;
}