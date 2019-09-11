//! A somewhat hacked-up version of a radio button using lensing.

use druid::widget::{Checkbox, Column, Label, Padding, Row};
use druid::{Data, Lens, LensWrap, LocalizedString, Widget};

struct EnumLens<T: Data> {
    variant: T,
}

impl<T: Data> EnumLens<T> {
    pub fn new(variant: T) -> Self {
        EnumLens { variant }
    }
}

impl<T: Data> Lens<T, bool> for EnumLens<T> {
    fn get<'a>(&self, data: &'a T) -> &'a bool {
        if data.same(&self.variant) {
            &true
        } else {
            &false
        }
    }

    fn with_mut<V, F: FnOnce(&mut bool) -> V>(&self, data: &mut T, f: F) -> V {
        let mut is_set = data.same(&self.variant);
        let val = f(&mut is_set);
        if is_set {
            *data = self.variant.clone()
        }
        val
    }
}

pub fn radio<T: Data + 'static>(
    variants: impl IntoIterator<Item = (T, LocalizedString<bool>)>,
) -> impl Widget<T> {
    let mut col = Column::new();
    for (variant, label) in variants.into_iter() {
        let mut row = Row::new();
        row.add_child(Checkbox::new(), 0.0);
        row.add_child(Label::new(label), 1.0);
        let lensed = LensWrap::new(row, EnumLens::new(variant));
        col.add_child(Padding::uniform(3.0, lensed), 0.0);
    }
    col
}
