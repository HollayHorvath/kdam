use crate::std_bar::Bar;

#[derive(Debug)]
/// Struct which implemented iterator trait and progress bar displays for `kdam::Bar`.
pub struct BarIterStruct<T> {
    /// Iterable to decorate with a progressbar.
    pub iterable: T,
    /// Instance of `kdam::Bar` to display progress updates for iterable.
    pub pb: Bar,
}

impl<T> std::ops::Deref for BarIterStruct<T> {
    type Target = Bar;

    fn deref(&self) -> &Self::Target {
        &self.pb
    }
}

impl<T> std::ops::DerefMut for BarIterStruct<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pb
    }
}

impl<T: ExactSizeIterator> ExactSizeIterator for BarIterStruct<T> {
    fn len(&self) -> usize {
        self.iterable.len()
    }
}

impl<S, T: Iterator<Item = S>> Iterator for BarIterStruct<T> {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iterable.next();
        if self.pb.internal.started {
            self.pb.update(1);
        } else {
            self.pb.refresh();
        }

        item
    }
}

impl<T: DoubleEndedIterator> DoubleEndedIterator for BarIterStruct<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = self.iterable.next_back();

        if self.pb.internal.started {
            self.pb.update(1);
        } else {
            self.pb.refresh();
        }

        item
    }
}

pub trait BarIter
where
    Self: Sized + Iterator,
{
    fn progress(self) -> BarIterStruct<Self>;
}

impl<S, T: Iterator<Item = S>> BarIter for T {
    /// Wrap any sized iterator to `kdam::BarIterStruct`.
    fn progress(self) -> BarIterStruct<Self> {
        let total = self.size_hint().0;
        BarIterStruct {
            iterable: self,
            pb: Bar {
                total: total as u64,
                ..Default::default()
            },
        }
    }
}
