/// n-dimensional position
pub type Position = [f32];

pub fn clone_slice_mut(s: &[f32]) -> Vec<f32> {
    let mut v = valloc(s.len());
    let c: &mut [f32] = v.as_mut_slice();
    /*for (i, e) in s.iter().enumerate() {
        c[i] = e.clone();
    }*/
    s.iter().zip(c.iter_mut()).for_each(|(e, c)| *c = e.clone());
    v
}

pub type Edge = (usize, usize);

pub enum Nodes {
    Mass(Vec<f32>),
    Degree(usize),
}

pub fn norm(n: &Position) -> f32 {
    n.iter().map(|i| i.clone().powi(2)).sum::<f32>().sqrt()
}

/// Allocate Vec without initializing
#[allow(clippy::uninit_vec)]
pub fn valloc(n: usize) -> Vec<f32> {
    let mut v = Vec::with_capacity(n);
    unsafe {
        v.set_len(n);
    }
    v
}

pub fn split_at_mut<T>(v: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let (first, second) = v.split_at_mut(mid);
    (first, second)
}

pub struct PointIter<'a> {
    pub dimensions: usize,
    pub offset: usize,
    pub list: &'a Vec<f32>,
}

impl<'a> PointIter<'a> {
    /// Returns a raw pointer to the next element, and increments the counter by `n`.
    ///
    /// # Safety
    /// Returned pointer may overflow the data.
    pub unsafe fn next_unchecked(&mut self, n: usize) -> *const f32 {
        let ptr = self.list.as_ptr().add(self.offset);
        self.offset += self.dimensions * n;
        ptr
    }
}

impl<'a> Iterator for PointIter<'a> {
    type Item = &'a [f32];

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.list.len() {
            return None;
        }
        let ret = {
            match self.list.get(self.offset..self.offset + self.dimensions) {
                Some(val) => val,
                None => return None,
            }
        };
        self.offset += self.dimensions;
        Some(ret)
    }
}

pub struct PointIterMut<'a> {
    pub dimensions: usize,
    pub offset: usize,
    pub list: &'a mut Vec<f32>,
}

impl<'a> Iterator for PointIterMut<'a> {
    type Item = &'a mut [f32];

    fn next<'b>(&'b mut self) -> Option<Self::Item> {
        if self.offset >= self.list.len() {
            return None;
        }
        let ret: &'b mut [f32] = {
            match self
                .list
                .get_mut(self.offset..self.offset + self.dimensions)
            {
                Some(val) => val,
                None => return None,
            }
        };
        self.offset += self.dimensions;
        Some(unsafe { std::mem::transmute(ret) })
    }
}

#[derive(Clone)]
pub struct PointList {
    /// Number of coordinates in a vector
    pub dimensions: usize,
    /// List of the coordinates of the vectors
    pub points: Vec<f32>,
}

impl<'a> PointList {
    pub fn get(&'a self, n: usize) -> &'a Position {
        let offset = n * self.dimensions;
        &self.points[offset..offset + self.dimensions]
    }

    pub fn get_clone(&self, n: usize) -> Vec<f32> {
        clone_slice_mut(self.get(n))
    }

    pub fn get_clone_slice(&self, n: usize, v: &mut [f32]) {
        v.clone_from_slice(self.get(n))
    }

    pub fn get_mut(&mut self, n: usize) -> &mut Position {
        let offset = n * self.dimensions;
        &mut self.points[offset..offset + self.dimensions]
    }

    /// n1 < n2
    pub fn get_2_mut(&mut self, n1: usize, n2: usize) -> Option<(&mut [f32], &mut [f32])> {
        let offset1 = n1 * self.dimensions;
        let offset2 = n2 * self.dimensions;
        {
            let (s1, s2) = split_at_mut(&mut self.points, offset2);
            match (
                s1.get_mut(offset1..offset1 + self.dimensions),
                s2.get_mut(..self.dimensions),
            ) {
                (Some(first), Some(second)) => Some((first, second)),
                _ => None,
            }
        }
    }

    pub fn set(&mut self, n: usize, val: &Position) {
        let offset = n * self.dimensions;
        self.points[offset..offset + self.dimensions].clone_from_slice(val);
    }

    pub fn iter(&self) -> PointIter {
        PointIter {
            dimensions: self.dimensions,
            list: &self.points,
            offset: 0,
        }
    }

    pub fn iter_from(&self, offset: usize) -> PointIter {
        PointIter {
            dimensions: self.dimensions,
            list: &self.points,
            offset: offset * self.dimensions,
        }
    }

    pub fn iter_mut(&mut self) -> PointIterMut {
        PointIterMut {
            dimensions: self.dimensions,
            list: &mut self.points,
            offset: 0,
        }
    }

    pub fn iter_mut_from(&mut self, offset: usize) -> PointIterMut {
        PointIterMut {
            dimensions: self.dimensions,
            list: &mut self.points,
            offset: offset * self.dimensions,
        }
    }

    pub fn remove(&mut self, mut offset: usize) {
        offset *= self.dimensions;
        let len = self.points.len();
        self.points
            .copy_within(offset + self.dimensions..len, offset);
        self.points.truncate(self.points.len() - self.dimensions);
    }
}

pub(crate) struct SendPtr<T>(pub std::ptr::NonNull<T>);

impl<T> Copy for SendPtr<T> {}
impl<T> Clone for SendPtr<T> {
    fn clone(&self) -> Self {
        SendPtr(self.0)
    }
}

unsafe impl<T> Send for SendPtr<T> {}
unsafe impl<T> Sync for SendPtr<T> {}
