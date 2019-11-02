#![no_std]

pub trait TupleTranspose {
    type Output;
    fn transpose(self) -> Self::Output;
}

#[rustfmt::skip]
mod impls {
    use super::TupleTranspose;

    macro_rules! define_tuple_transpose {
        ($($v:ident: $T:ident),+ $(,)?) => {
            impl<$($T),+, E> TupleTranspose for ($(Result<$T, E>,)+) {
                type Output = Result<($($T,)+), E>;
                fn transpose(self) -> Self::Output {
                    let ($($v,)+) = self;
                    Ok(($($v?,)+))
                }
            }

            impl<$($T),+> TupleTranspose for ($(Option<$T>,)+) {
                type Output = Option<($($T,)+)>;
                fn transpose(self) -> Self::Output {
                    let ($($v,)+) = self;
                    Some(($($v?,)+))
                }
            }
        }
    }

    define_tuple_transpose!(v0: T0);
    define_tuple_transpose!(v0: T0, v1: T1);
    define_tuple_transpose!(v0: T0, v1: T1, v2: T2);
    define_tuple_transpose!(v0: T0, v1: T1, v2: T2, v3: T3);
    define_tuple_transpose!(v0: T0, v1: T1, v2: T2, v3: T3, v4: T4);
    define_tuple_transpose!(v0: T0, v1: T1, v2: T2, v3: T3, v4: T4, v5: T5);
    define_tuple_transpose!(v0: T0, v1: T1, v2: T2, v3: T3, v4: T4, v5: T5, v6: T6);
    define_tuple_transpose!(v0: T0, v1: T1, v2: T2, v3: T3, v4: T4, v5: T5, v6: T6, v7: T7);
    define_tuple_transpose!(
        v0: T0, v1: T1, v2: T2, v3: T3, v4: T4, v5: T5, v6: T6, v7: T7,
        v8: T8,
    );
    define_tuple_transpose!(
        v0: T0, v1: T1, v2: T2, v3: T3, v4: T4, v5: T5, v6: T6, v7: T7,
        v8: T8, v9: T9,
    );
    define_tuple_transpose!(
        v0: T0, v1: T1, v2: T2, v3: T3, v4: T4, v5: T5, v6: T6, v7: T7,
        v8: T8, v9: T9, v10: T10,
    );
    define_tuple_transpose!(
        v0: T0, v1: T1, v2: T2, v3: T3, v4: T4, v5: T5, v6: T6, v7: T7,
        v8: T8, v9: T9, v10: T10, v11: T11,
    );
    define_tuple_transpose!(
        v0: T0, v1: T1, v2: T2, v3: T3, v4: T4, v5: T5, v6: T6, v7: T7,
        v8: T8, v9: T9, v10: T10, v11: T11, v12: T12,
    );
    define_tuple_transpose!(
        v0: T0, v1: T1, v2: T2, v3: T3, v4: T4, v5: T5, v6: T6, v7: T7,
        v8: T8, v9: T9, v10: T10, v11: T11, v12: T12, v13: T13,
    );
    define_tuple_transpose!(
        v0: T0, v1: T1, v2: T2, v3: T3, v4: T4, v5: T5, v6: T6, v7: T7,
        v8: T8, v9: T9, v10: T10, v11: T11, v12: T12, v13: T13, v14: T14,
    );
    define_tuple_transpose!(
        v0: T0, v1: T1, v2: T2, v3: T3, v4: T4, v5: T5, v6: T6, v7: T7,
        v8: T8, v9: T9, v10: T10, v11: T11, v12: T12, v13: T13, v14: T14, v15: T15,
    );
    define_tuple_transpose!(
        v0: T0, v1: T1, v2: T2, v3: T3, v4: T4, v5: T5, v6: T6, v7: T7,
        v8: T8, v9: T9, v10: T10, v11: T11, v12: T12, v13: T13, v14: T14, v15: T15, v16: T16,
    );
}

#[cfg(test)]
mod tests {
    use super::TupleTranspose;

    #[test]
    fn test_ok_ok() {
        assert_eq!(
            (Ok::<_, ()>(1u32), Ok(2.0f32)).transpose(),
            Ok((1u32, 2.0f32))
        );
    }

    #[test]
    fn test_ok_err() {
        assert_eq!((Ok(1u32), Err::<u64, _>(2.0f32)).transpose(), Err(2.0f32));
    }

    #[test]
    fn test_err_ok() {
        assert_eq!((Err::<i32, _>(1u32), Ok(2.0f32)).transpose(), Err(1u32));
    }

    #[test]
    fn test_some_some() {
        assert_eq!((Some(1u32), Some(2.0f32)).transpose(), Some((1u32, 2.0f32)));
    }

    #[test]
    fn test_some_none() {
        assert_eq!((Some(1u32), None::<f32>).transpose(), None::<(u32, f32)>);
    }

    #[test]
    fn test_none_some() {
        assert_eq!((None::<u32>, Some(2.0f32)).transpose(), None::<(u32, f32)>);
    }
}
