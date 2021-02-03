use std::marker::{PhantomData, Unsize};


pub struct Arena<B>
    where B: ?Sized {
    elements: Vec<Box<B>>,
}

impl<B> Arena<B>
    where B: ?Sized {

    pub fn new() -> Self {
        return Self {
            elements: Vec::new(),
        };
    }

    pub fn insert<E>(&mut self, element: E) -> Ref<E>
        where E: Unsize<B> + 'static {
        self.elements.push(Box::<E>::new(element));
        return Ref {
            index: self.elements.len() - 1,
            phantom: PhantomData::default(),
        };
    }

    pub fn resolve<'e, E>(&self, r: &'e Ref<E>) -> &'e E
        where E: Unsize<B> {
        let element: &B = self.elements[r.index].as_ref();
        return unsafe { &*(element as *const B as *const E) };
    }

    pub fn iter(&self) -> impl Iterator<Item=&B> {
        return self.elements.iter().map(Box::as_ref);
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut B> {
        return self.elements.iter_mut().map(Box::as_mut);
    }
}

// TODO: Bind the lifetime of the Ref to the Arena
pub struct Ref<E> {
    // Index of the element in the arena
    index: usize,

    phantom: PhantomData<E>,
}