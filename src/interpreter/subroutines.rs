use crate::types::coordinate::Coordinate;
use crate::types::direction::Direction;
use std::fmt;

#[derive(Debug, Clone)]
pub struct StackFrame {
    /// Position to return to
    pub return_position: Coordinate,
    /// Direction after return
    pub return_direction: Direction,
}

impl StackFrame {
    pub fn new(return_position: Coordinate, return_direction: Direction) -> Self {
        StackFrame {
            return_position,
            return_direction,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CallStack {
    /// Stack of return positions
    pub frames: Vec<StackFrame>,
    /// Maximum depth reached
    pub max_depth: usize,
}

impl CallStack {
    pub fn new() -> Self {
        CallStack {
            frames: Vec::new(),
            max_depth: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        CallStack {
            frames: Vec::with_capacity(capacity),
            max_depth: 0,
        }
    }

    pub fn push(&mut self, frame: StackFrame) {
        self.frames.push(frame);
        self.max_depth = self.max_depth.max(self.frames.len());
    }

    pub fn push_return(&mut self, return_position: Coordinate, return_direction: Direction) {
        self.push(StackFrame::new(return_position, return_direction));
    }

    pub fn pop(&mut self) -> Option<StackFrame> {
        self.frames.pop()
    }

    pub fn pop_or_none(&mut self) -> Option<StackFrame> {
        self.pop()
    }

    pub fn peek(&self) -> Option<&StackFrame> {
        self.frames.last()
    }

    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    pub fn len(&self) -> usize {
        self.frames.len()
    }

    pub fn depth(&self) -> usize {
        self.frames.len()
    }

    pub fn clear(&mut self) {
        self.frames.clear();
    }

    pub fn truncate(&mut self, new_len: usize) {
        if new_len < self.frames.len() {
            self.frames.truncate(new_len);
        }
    }

    pub fn get(&self, index: usize) -> Option<&StackFrame> {
        self.frames.get(index)
    }

    pub fn get_from_top(&self, index_from_top: usize) -> Option<&StackFrame> {
        if index_from_top >= self.frames.len() {
            None
        } else {
            self.frames.get(self.frames.len() - 1 - index_from_top)
        }
    }

    pub fn swap_top_two(&mut self) -> bool {
        if self.frames.len() < 2 {
            false
        } else {
            let len = self.frames.len();
            self.frames.swap(len - 1, len - 2);
            true
        }
    }

    pub fn as_slice(&self) -> &[StackFrame] {
        &self.frames
    }

    pub fn is_within_limit(&self, limit: usize) -> bool {
        self.frames.len() <= limit
    }

    pub fn max_depth_reached(&self) -> usize {
        self.max_depth
    }

    pub fn iter(&self) -> impl Iterator<Item = &StackFrame> {
        self.frames.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut StackFrame> {
        self.frames.iter_mut()
    }

    pub fn contains_position(&self, position: Coordinate) -> bool {
        self.frames.iter().any(|frame| frame.return_position == position)
    }

    pub fn filter_by_direction(&self, direction: Direction) -> Vec<&StackFrame> {
        self.frames
            .iter()
            .filter(|frame| frame.return_direction == direction)
            .collect()
    }
}

impl Default for CallStack {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CallStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, frame) in self.frames.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "({} {})", frame.return_position, frame.return_direction)?;
        }
        write!(f, "]")
    }
}

impl From<Vec<StackFrame>> for CallStack {
    fn from(frames: Vec<StackFrame>) -> Self {
        let max_depth = frames.len();
        CallStack {
            frames,
            max_depth,
        }
    }
}

impl From<Vec<(Coordinate, Direction)>> for CallStack {
    fn from(pairs: Vec<(Coordinate, Direction)>) -> Self {
        let frames: Vec<StackFrame> = pairs
            .into_iter()
            .map(|(pos, dir)| StackFrame::new(pos, dir))
            .collect();
        frames.into()
    }
}