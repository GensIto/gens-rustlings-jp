use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: この関数はいつも`Ok`を返すべきではない。
        if value == 0 {
            return Err(CreationError::Zero);
        } else if value < 0 {
            return Err(CreationError::Negative);
        }

        Ok(Self(value as u64))

        // use std::cmp::Orderingは標準ライブラリで 比較結果を表す列挙型らしい
        // 標準ライブラリ何あるか覚えないといけないな~
        // match value.cmp(&0) {
        //     Ordering::Less => Err(CreationError::Negative),
        //     Ordering::Equal => Err(CreationError::Zero),
        //     Ordering::Greater => Ok(Self(value as u64)),
        // }
    }
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
