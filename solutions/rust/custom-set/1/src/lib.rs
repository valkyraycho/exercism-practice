#[derive(Debug)]
pub struct CustomSet<T> {
    inner: Vec<T>,
}

impl<T: PartialEq + Clone> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.len() == other.inner.len() && self.is_subset(other)
    }
}

impl<T: PartialEq + Clone> Eq for CustomSet<T> {}

impl<T> CustomSet<T>
where
    T: PartialEq + Clone,
{
    pub fn new(input: &[T]) -> Self {
        let mut vec = vec![];
        for data in input {
            if !vec.contains(data) {
                vec.push(data.clone());
            }
        }
        Self { inner: vec }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.inner.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.inner.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.inner.iter().all(|element| other.contains(element))
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        other.inner.iter().all(|element| !self.contains(element))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            inner: self
                .inner
                .iter()
                .filter(|&element| other.contains(element))
                .cloned()
                .collect(),
        }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        Self {
            inner: self
                .inner
                .iter()
                .filter(|&element| !other.contains(element))
                .cloned()
                .collect(),
        }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self {
            inner: self
                .inner
                .iter()
                .chain(
                    other
                        .inner
                        .iter()
                        .filter(|&element| !self.contains(element)),
                )
                .cloned()
                .collect(),
        }
    }
}
