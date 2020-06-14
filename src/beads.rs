use crate::grid::Grid;

pub type Beads<T> = Vec<(T, usize)>;

struct BeadsBuilder<T> {
    data: Vec<(T,usize)>,
    item: T,
    count: usize,
}

pub fn create_beads<T: Clone + Eq>(data: &[T]) -> Beads<T> {
    let mut builder = BeadsBuilder::new(data[0].clone());
    &data[1..].iter().for_each(|item|{ builder.add(item.clone()) });
    builder.into()
}

impl<T> Into<Beads<T>> for BeadsBuilder<T> {
    fn into(self) -> Beads<T> {
        let BeadsBuilder {mut data, item, count} = self;
        data.push((item, count));
        data
    }
}

impl<T: Eq> BeadsBuilder<T> {
    fn new(item: T) -> Self {
        Self {
            data: Vec::new(),
            item,
            count: 1,
        }
    }
    fn add(&mut self, item: T) {
        if self.item == item {
            self.count += 1
        } else {
            let finished_item = std::mem::replace(&mut self.item,item);
            self.data.push((finished_item, self.count));
            self.count = 1;
        }
    }
}
