pub trait EventQueue {
    fn enqueue<T>(&self, event: T);
}
