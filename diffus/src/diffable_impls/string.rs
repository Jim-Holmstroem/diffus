use crate::{edit::collection, edit::Edit, lcs::Lcs, Diffable};

impl<'a> Diffable<'a> for String {
    type D = Box<dyn Iterator<Item = collection::Edit<char>> + 'a>;

    fn diff(&'a self, other: &'a Self) -> Edit<'a, Self> {
        let (s, modified) = Lcs::new(
            self.chars(),
            || other.chars(),
            self.chars().count(),
            other.chars().count(),
        )
        .diff(self.chars(), other.chars());

        if modified {
            Edit::Change(s)
        } else {
            Edit::Copy
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{collection::Edit::*, *};

    #[test]
    fn diff() {
        use super::Diffable;

        let left = "XMJYAUZ".to_owned();
        let right = "MZJAWXU".to_owned();

        let diff = left.diff(&right);
        if let Edit::Change(diff) = diff {
            assert_eq!(
                diff.collect::<Vec<_>>(),
                vec![
                    Remove('X'),
                    Copy('M'),
                    Insert('Z'),
                    Copy('J'),
                    Remove('Y'),
                    Copy('A'),
                    Insert('W'),
                    Insert('X'),
                    Copy('U'),
                    Remove('Z')
                ]
            );
        } else {
            unreachable!()
        }
    }
}