pub mod avl_tree {
    use replace_with::replace_with;
    use std::fmt;

pub trait Monoid {
    const MEMPTY: Self;
    fn mappend(self, other: Self) -> Self;
}

#[derive(Clone, PartialEq, Eq)]
pub struct Node<T: Monoid + Copy + Eq + fmt::Display + Ord> {
    val: T,
    left: Box<AVLTree<T>>,
    right: Box<AVLTree<T>>,
    height: usize,
    size: usize,
    sum: T,
}

#[derive(Clone, PartialEq, Eq)]
pub enum AVLTree<T: Monoid + Copy + Eq + fmt::Display + Ord> {
    None(),
    Node(Node<T>),
}

impl<T: Monoid + Copy + Eq + fmt::Display + Ord> fmt::Display for AVLTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AVLTree::None() => {
                write!(f, "")
            }
            AVLTree::Node(Node {
                val, left, right, ..
            }) => {
                let lft = match &**left {
                    AVLTree::None() => String::new(),
                    AVLTree::Node(Node {
                        val: l_val,
                        left: l_left,
                        right: l_right,
                        ..
                    }) => {
                        format!("[ {} {} {} ]", l_left, l_val, l_right)
                    }
                };
                let rht = match &**right {
                    AVLTree::None() => String::new(),
                    AVLTree::Node(Node {
                        val: r_val,
                        left: r_left,
                        right: r_right,
                        ..
                    }) => {
                        format!("[ {} {} {} ]", r_left, r_val, r_right)
                    }
                };
                write!(f, "{}", dels(format!("[ {} {} {} ]", lft, val, rht)))
            }
        }
    }
}

impl<T: Monoid + Copy + Eq + fmt::Display + Ord> fmt::Debug for AVLTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AVLTree::None() => {
                write!(f, "")
            }
            AVLTree::Node(Node {
                val,
                left,
                right,
                height,
                ..
            }) => {
                let lft = match &**left {
                    AVLTree::None() => String::new(),
                    AVLTree::Node(Node {
                        val: l_val,
                        left: l_left,
                        right: l_right,
                        height: l_height,
                        ..
                    }) => {
                        format!("[ {:?} ({},{}) {:?} ]", l_left, l_val, l_height, l_right)
                    }
                };
                let rht = match &**right {
                    AVLTree::None() => String::new(),
                    AVLTree::Node(Node {
                        val: r_val,
                        left: r_left,
                        right: r_right,
                        height: r_height,
                        ..
                    }) => {
                        format!("[ {:?} ({},{}) {:?} ]", r_left, r_val, r_height, r_right)
                    }
                };
                write!(
                    f,
                    "{}",
                    dels(format!("[ {} ({},{}) {} ]", lft, val, height, rht))
                )
            }
        }
    }
}

//собираем дерево из верщины и левого и правого поддеревьев
impl<T: Monoid + Copy + Eq + fmt::Display + Ord> From<(T, AVLTree<T>, AVLTree<T>)> for AVLTree<T> {
    fn from((val, left, right): (T, AVLTree<T>, AVLTree<T>)) -> Self {
        let mut tree = AVLTree::Node(Node {
            val,
            sum: T::MEMPTY,
            left: Box::new(left),
            right: Box::new(right),
            height: 0,
            size: 0,
        });
        tree.update();
        tree
    }
}

    
//собираем дерево по всем параметрам
impl<T: Monoid + Copy + Eq + fmt::Display + Ord> From<(T, AVLTree<T>, AVLTree<T>, usize, usize, T)>
    for AVLTree<T>
{
    fn from((val, left, right, height, size, sum): (T, AVLTree<T>, AVLTree<T>, usize, usize, T)) -> Self {
        AVLTree::Node(Node {
            val,
            left: Box::new(left),
            right: Box::new(right),
            height,
            size,
            sum,
        })
    }
}

impl<T: Monoid + Copy + Eq + fmt::Display + Ord> From<T> for AVLTree<T> {
    fn from(val: T) -> Self {
        AVLTree::Node(Node {
            val,
            left: Box::new(AVLTree::None()),
            right: Box::new(AVLTree::None()),
            height: 1,
            size: 1,
            sum: val,
        })
    }
}

impl<T: Monoid + Copy + Eq + fmt::Display + Ord> From<(AVLTree<T>, AVLTree<T>)> for AVLTree<T> {
    fn from((left, right): (AVLTree<T>, AVLTree<T>)) -> Self {
        let mut right = right;
        let vert = right.min();
        right.delete(vert);
        AVLTree::from((vert, left, right))
    }
}

impl<T: Monoid + Copy + Eq + fmt::Display + Ord> AVLTree<T> {
    fn destruct(self) -> (T, AVLTree<T>, AVLTree<T>) {
        if let AVLTree::Node(Node {
            val, left, right, ..
        }) = self
        {
            return (val, *left, *right);
        }
        panic!("failed to destruct AVLTree enum, expected AVLTree::Node(Node), found AVLTree::None");
    }
    fn height(&self) -> usize {
        match self {
            AVLTree::None() => 0usize,
            AVLTree::Node(Node { height, .. }) => *height,
        }
    }
    fn size(&self) -> usize {
        match self {
            AVLTree::None() => 0usize,
            AVLTree::Node(Node { size, .. }) => *size,
        }
    }
    fn sum(&self) -> T {
        match self {
            AVLTree::None() => T::MEMPTY,
            AVLTree::Node(Node { sum, .. }) => *sum,
        }
    }
    pub fn mappend_from_to(&self, min: T, max: T) -> T {
        match self {
            AVLTree::None() => {
                T::MEMPTY
            }
            AVLTree::Node(Node {
                val,
                left,
                right,
                sum,
                ..
            }) => {
                if **left != AVLTree::None() && left.max() < min {
                    if *val == min {
                        return Monoid::mappend(*val, right.mappend_from_to(min, max));
                    } else {
                        return right.mappend_from_to(min, max);
                    }
                }
                if **right != AVLTree::None() && max < right.min() {
                    if *val == max {
                        return Monoid::mappend(*val, left.mappend_from_to(min, max));
                    } else {
                        return left.mappend_from_to(min, max);
                    }
                }
                if **left != AVLTree::None()
                    && **right != AVLTree::None()
                    && min <= left.min()
                    && right.max() < max
                {
                    return *sum;
                }
                Monoid::mappend(
                    *val,
                    Monoid::mappend(
                        left.mappend_from_to(min, max),
                        right.mappend_from_to(min, max),
                    ),
                )
            }
        }
    }
    fn balance(&mut self) {
        replace_with(self, || AVLTree::None(), |self_| self_.rotate());
    }
    fn rotate(self) -> Self {
        if self == AVLTree::None() {
            return AVLTree::None();
        }
        let (val, left, right) = self.destruct();
        let mut ans;
        if left.height() > right.height() && left.height() - right.height() > 1 {
            let (d_val, d_left, d_right) = left.destruct();
            if d_left.height() >= d_right.height() {
                let mut right_t = AVLTree::from((val, d_right, right));
                right_t.update();
                ans = AVLTree::from((d_val, d_left, right_t));
                ans.update();
                return ans;
            } else {
    
                let (dd_val, dd_left, dd_right) = d_right.destruct();
                let mut left_t = AVLTree::from((d_val, d_left, dd_left));
                left_t.update();
                let mut right_t = AVLTree::from((val, dd_right, right));
                right_t.update();
                ans = AVLTree::from((dd_val, left_t, right_t));
                ans.update();
                return ans;
            }
        } else if left.height() < right.height() && right.height() - left.height() > 1 {
            let (d_val, d_left, d_right) = right.destruct();
            if d_right.height() >= d_left.height() {
                let mut left_t = AVLTree::from((val, left, d_left));
                left_t.update();
                ans = AVLTree::from((d_val, left_t, d_right));
                ans.update();
                return ans;
            } else {
                let (dd_val, dd_left, dd_right) = d_left.destruct();
                let mut left_t = AVLTree::from((val, left, dd_left));
                left_t.update();
                let mut right_t = AVLTree::from((d_val, dd_right, d_right));
                right_t.update();
                ans = AVLTree::from((dd_val, left_t, right_t));
                ans.update();
                return ans;
            }
        }
        let mut ans = AVLTree::from((val, left, right));
        ans.update();
        ans
    }
    pub fn insert(&mut self, i: T) {
        replace_with(self, || AVLTree::None(), |self_| self_.ins(i));
    }
    fn ins(self, i: T) -> Self {
        match self {
            AVLTree::None() => {
                AVLTree::from(i)
            }
            _ => {
                let (val, left, right) = self.destruct();
                if i == val {
                    let mut ans = AVLTree::from((val, left, right));
                    ans.update();
                    return ans;
                }
                let mut t_left = left;
                let mut t_right = right;
                if i < val {
                    t_left.insert(i);
                } else {
                    t_right.insert(i);
                }
                let mut ans = AVLTree::from((val, t_left, t_right));
                ans.balance();
                ans.update();
                ans
            }
        }
    }
    pub fn index(&self, i: usize) -> T {
        match self {
            AVLTree::None() => {
                panic!("wrong index, failed to access");
            }
            AVLTree::Node(Node {
                val, left, right, ..
            }) => {
                if i == left.size() {
                    return *val;
                }
                if i < left.size() {
                    left.index(i)
                } else {
                    right.index(i - left.size() - 1)
                }
            }
        }
    }
    fn min(&self) -> T {
        match self {
            AVLTree::None() => panic!("tried to find min in empty tree"),
            AVLTree::Node(Node { val, left, .. }) => {
                if **left == AVLTree::None() {
                    return *val;
                }
                left.min()
            }
        }
    }
    fn max(&self) -> T {
        match self {
            AVLTree::None() => panic!("tried to find max in empty tree"),
            AVLTree::Node(Node { val, right, .. }) => {
                if **right == AVLTree::None() {
                    return *val;
                }
                right.max()
            }
        }
    }
    fn delete(&mut self, i: T) {
        replace_with(self, || AVLTree::None(), |self_| self_.del(i));
    }
    fn del(self, i: T) -> Self {
        if let AVLTree::None() = self {
            return self;
        }
        let (val, mut left, mut right) = self.destruct();
        let mut vert = val;
        if val == i {
            if left == AVLTree::None() && right == AVLTree::None() {
                return AVLTree::None();
            }
            if right != AVLTree::None() {
                vert = right.min();
                right.delete(vert);
            } else {
                vert = left.max();
    
                left.delete(vert);
            }
        }
        if i < val {
            left.delete(i);
        } else {
            right.delete(i);
        }
        AVLTree::from((vert, left, right))
    }
    pub fn find(&self, i: T) -> bool {
        match self {
            AVLTree::None() => false,
            AVLTree::Node(Node {
                val, left, right, ..
            }) => {
                if i == *val {
                    return true;
                }
                if i > *val {
                    right.find(i)
                } else {
                    left.find(i)
                }
            }
        }
    }
    fn update(&mut self) {
        replace_with(self, || AVLTree::None(), |self_| self_.upd());
    }
    fn upd(self) -> Self {
        if let AVLTree::None() = self {
            return AVLTree::None();
        }
        let (val, left, right) = self.destruct();
        let height = left.height().max(right.height()) + 1;
        let size = left.size() + right.size() + 1;
        let sum = Monoid::mappend(val, Monoid::mappend(left.sum(), right.sum()));
        AVLTree::from((val, left, right, height, size, sum))
    }
    pub fn avl_merge(left: AVLTree<T>, right: AVLTree<T>) -> Self {
        if left == AVLTree::None() && right == AVLTree::None() {
            return AVLTree::None();
        }
        if left != AVLTree::None() {
            let mut lft = left;
            let vert = lft.max();
            lft.delete(vert);
            return AVLTree::avl_merge_with_root(vert, lft, right);
        }
        let mut righ = right;
        let vert = righ.min();
        righ.delete(vert);
        AVLTree::avl_merge_with_root(vert, left, righ)
    }
    pub fn avl_merge_with_root(vert: T, left: AVLTree<T>, right: AVLTree<T>) -> Self {
        if (left.height() as i32 - right.height() as i32).abs() <= 1 {
            return AVLTree::from((vert, left, right));
        }
        if left.height() > right.height() {
            let (var, left_t, mut right_t) = left.destruct();
            right_t = AVLTree::avl_merge_with_root(vert, right_t, right);
            let mut tree = AVLTree::from((var, left_t, right_t));
            tree.balance();
            tree.update();
            tree
        } else {
            let (var, mut left_t, right_t) = right.destruct();
            left_t = AVLTree::avl_merge_with_root(vert, left, left_t);
            let mut tree = AVLTree::from((var, left_t, right_t));
            tree.balance();
            tree.update();
            tree
        }
    }
    pub fn divide(self, k: T) -> (Self, Self) {
        if self == AVLTree::None() {
            return (AVLTree::None(), AVLTree::None());
        }
        let (vert, left, right) = self.destruct();
        if k < vert {
            let mut right_t = AVLTree::avl_merge(AVLTree::from(vert), right);
            let (left_t, right_d) = left.divide(k);
            right_t = AVLTree::avl_merge(right_d, right_t);
            right_t.balance();
            (left_t, right_t)
        } else {
            let mut left_t = AVLTree::avl_merge(left, AVLTree::from(vert));
            let (left_d, right_t) = right.divide(k);
            left_t = AVLTree::avl_merge(left_t, left_d);
            left_t.balance();
            (left_t, right_t)
        }
    }
    pub fn in_order(&self) -> String {
        match self {
            AVLTree::None() => String::new(),
            AVLTree::Node(Node {
                val, left, right, ..
            }) => {
                format!("{} {} {}", (*left).in_order(), val, (*right).in_order(),)
            }
        }
    }
    pub fn pre_order(&self) -> String {
        match self {
            AVLTree::None() => String::new(),
            AVLTree::Node(Node {
                val, left, right, ..
            }) => {
                format!("{} {} {}", val, (*left).in_order(), (*right).in_order(),)
            }
        }
    }
    pub fn post_order(&self) -> String {
        match self {
            AVLTree::None() => String::new(),
            AVLTree::Node(Node {
                val, left, right, ..
            }) => {
                    
                format!("{} {} {}", (*left).in_order(), (*right).in_order(), val)
            }
        }
    }
}

fn dels(string: String) -> String {
    string
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
}

#[cfg(test)]
mod test {
    // use super::avl_tree::AVLTree;
    // use rand::{thread_rng, Rng};

    #[test]
    fn test() {
    }
}