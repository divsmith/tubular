use crate::types::bigint::TubularBigInt;
use std::fmt;

#[derive(Debug, Clone)]
pub struct DataStack {
    /// Stack values (arbitrary precision integers)
    pub data: Vec<TubularBigInt>,
    /// Maximum depth reached (for monitoring)
    pub max_depth: usize,
}

impl DataStack {
    pub fn new() -> Self {
        DataStack {
            data: Vec::new(),
            max_depth: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        DataStack {
            data: Vec::with_capacity(capacity),
            max_depth: 0,
        }
    }

    pub fn push(&mut self, value: TubularBigInt) {
        self.data.push(value);
        self.max_depth = self.max_depth.max(self.data.len());
    }

    pub fn pop(&mut self) -> TubularBigInt {
        self.data.pop().unwrap_or_else(|| TubularBigInt::zero())
    }

    pub fn pop_or_zero(&mut self) -> TubularBigInt {
        self.pop()
    }

    pub fn peek(&self) -> TubularBigInt {
        self.data.last().cloned().unwrap_or_else(|| TubularBigInt::zero())
    }

    pub fn peek_depth(&self, depth: usize) -> TubularBigInt {
        if depth >= self.data.len() {
            TubularBigInt::zero()
        } else {
            self.data[self.data.len() - 1 - depth].clone()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn depth(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn truncate(&mut self, new_len: usize) {
        if new_len < self.data.len() {
            self.data.truncate(new_len);
        }
    }

    pub fn swap_top_two(&mut self) -> bool {
        if self.data.len() < 2 {
            false
        } else {
            let len = self.data.len();
            self.data.swap(len - 1, len - 2);
            true
        }
    }

    pub fn duplicate(&mut self) -> bool {
        if self.data.is_empty() {
            false
        } else {
            let top = self.data.last().unwrap().clone();
            self.push(top);
            true
        }
    }

    pub fn pop_n(&mut self, n: usize) -> Vec<TubularBigInt> {
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            result.push(self.pop());
        }
        result
    }

    pub fn push_n(&mut self, values: Vec<TubularBigInt>) {
        for value in values {
            self.push(value);
        }
    }

    pub fn get(&self, index: usize) -> Option<&TubularBigInt> {
        self.data.get(index)
    }

    pub fn get_from_top(&self, index_from_top: usize) -> Option<&TubularBigInt> {
        if index_from_top >= self.data.len() {
            None
        } else {
            self.data.get(self.data.len() - 1 - index_from_top)
        }
    }

    pub fn as_slice(&self) -> &[TubularBigInt] {
        &self.data
    }

    pub fn is_within_limit(&self, limit: usize) -> bool {
        self.data.len() <= limit
    }

    pub fn max_depth_reached(&self) -> usize {
        self.max_depth
    }
}

impl Default for DataStack {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DataStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, value) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", value)?;
        }
        write!(f, "]")
    }
}

impl From<Vec<TubularBigInt>> for DataStack {
    fn from(values: Vec<TubularBigInt>) -> Self {
        let max_depth = values.len();
        DataStack {
            data: values,
            max_depth,
        }
    }
}

impl From<Vec<i64>> for DataStack {
    fn from(values: Vec<i64>) -> Self {
        let bigint_values: Vec<TubularBigInt> = values
            .into_iter()
            .map(TubularBigInt::new)
            .collect();
        bigint_values.into()
    }
}