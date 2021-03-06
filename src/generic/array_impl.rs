use super::Array;
use traits::{Stringify, Append};

impl Array {
    pub fn new<I>(values: I) -> Self
        where I: Into<Array>
    {
        values.into()
    }

    pub fn dtype(&self) -> String {
        match self {
            &Array::Int64Array(_) => "i64".to_string(),
            &Array::Int32Array(_) => "i32".to_string(),
            &Array::Int16Array(_) => "i16".to_string(),
            &Array::Int8Array(_) => "i8".to_string(),
            &Array::IsizeArray(_) => "isize".to_string(),
            &Array::UInt64Array(_) => "u64".to_string(),
            &Array::UInt32Array(_) => "u32".to_string(),
            &Array::UInt16Array(_) => "u16".to_string(),
            &Array::UInt8Array(_) => "u8".to_string(),
            &Array::UsizeArray(_) => "usize".to_string(),
            &Array::Float64Array(_) => "f64".to_string(),
            &Array::Float32Array(_) => "f32".to_string(),
            &Array::BoolArray(_) => "bool".to_string(),
            &Array::StringArray(_) => "str".to_string(),
        }
    }

    pub fn is_numeric(&self) -> bool {
        match self {
            &Array::Int64Array(_) => true,
            &Array::Int32Array(_) => true,
            &Array::Int16Array(_) => true,
            &Array::Int8Array(_) => true,
            &Array::IsizeArray(_) => true,
            &Array::UInt64Array(_) => true,
            &Array::UInt32Array(_) => true,
            &Array::UInt16Array(_) => true,
            &Array::UInt8Array(_) => true,
            &Array::UsizeArray(_) => true,
            &Array::Float64Array(_) => true,
            &Array::Float32Array(_) => true,
            &Array::BoolArray(_) => false,
            &Array::StringArray(_) => false,
        }
    }
}

impl Stringify for Array {
    fn into_string_vec(&self) -> Vec<String> {
        match self {
            &Array::Int64Array(ref vals) => vals.into_string_vec(),
            &Array::Int32Array(ref vals) => vals.into_string_vec(),
            &Array::Int16Array(ref vals) => vals.into_string_vec(),
            &Array::Int8Array(ref vals) => vals.into_string_vec(),
            &Array::IsizeArray(ref vals) => vals.into_string_vec(),
            &Array::UInt64Array(ref vals) => vals.into_string_vec(),
            &Array::UInt32Array(ref vals) => vals.into_string_vec(),
            &Array::UInt16Array(ref vals) => vals.into_string_vec(),
            &Array::UInt8Array(ref vals) => vals.into_string_vec(),
            &Array::UsizeArray(ref vals) => vals.into_string_vec(),
            &Array::Float64Array(ref vals) => vals.into_string_vec(),
            &Array::Float32Array(ref vals) => vals.into_string_vec(),
            &Array::BoolArray(ref vals) => vals.into_string_vec(),
            &Array::StringArray(ref vals) => vals.into_string_vec(),
        }
    }
}

impl Append for Array {
    fn append(&self, other: &Array) -> Array {

        match (self, other) {
            (&Array::Int64Array(ref l), &Array::Int64Array(ref r)) => {
                Array::Int64Array(l.append(r))
            }
            (&Array::Int32Array(ref l), &Array::Int32Array(ref r)) => {
                Array::Int32Array(l.append(r))
            }
            (&Array::Int16Array(ref l), &Array::Int16Array(ref r)) => {
                Array::Int16Array(l.append(r))
            }
            (&Array::Int8Array(ref l), &Array::Int8Array(ref r)) => Array::Int8Array(l.append(r)),
            (&Array::IsizeArray(ref l), &Array::IsizeArray(ref r)) => {
                Array::IsizeArray(l.append(r))
            }
            (&Array::UInt64Array(ref l), &Array::UInt64Array(ref r)) => {
                Array::UInt64Array(l.append(r))
            }
            (&Array::UInt32Array(ref l), &Array::UInt32Array(ref r)) => {
                Array::UInt32Array(l.append(r))
            }
            (&Array::UInt16Array(ref l), &Array::UInt16Array(ref r)) => {
                Array::UInt16Array(l.append(r))
            }
            (&Array::UInt8Array(ref l), &Array::UInt8Array(ref r)) => {
                Array::UInt8Array(l.append(r))
            }
            (&Array::UsizeArray(ref l), &Array::UsizeArray(ref r)) => {
                Array::UsizeArray(l.append(r))
            }
            (&Array::Float64Array(ref l), &Array::Float64Array(ref r)) => {
                Array::Float64Array(l.append(r))
            }
            (&Array::Float32Array(ref l), &Array::Float32Array(ref r)) => {
                Array::Float32Array(l.append(r))
            }
            (&Array::BoolArray(ref l), &Array::BoolArray(ref r)) => Array::BoolArray(l.append(r)),
            (&Array::StringArray(ref l), &Array::StringArray(ref r)) => {
                Array::StringArray(l.append(r))
            }
            (_, _) => panic!(""),
        }
    }
}


#[cfg(test)]
mod tests {

    use generic::Array;
    use nullvec::NullVec;
    use traits::Stringify;

    #[test]
    fn test_into_string_vec() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let arr: Array = nvec.into();
        let exp = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        assert_eq!(arr.into_string_vec(), exp);

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![false, false, true]));
        let arr: Array = nvec.into();
        let exp = vec!["1".to_string(), "2".to_string(), "Null".to_string()];
        assert_eq!(arr.into_string_vec(), exp);

        let values: Vec<String> = vec!["a".to_string(), "bb".to_string(), "".to_string()];
        let nvec = NullVec::with_mask(values, Some(vec![false, false, true]));
        let arr: Array = nvec.into();
        let exp = vec!["a".to_string(), "bb".to_string(), "Null".to_string()];
        assert_eq!(arr.into_string_vec(), exp);
    }
}
