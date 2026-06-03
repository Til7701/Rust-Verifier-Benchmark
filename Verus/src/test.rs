verus! {

// struct LinkedList<T: Sized> {
//     head: Option<Box<Node<T>>>,
// }
//
// struct Node<T> {
//     e: T,
//     next: Option<Box<Node<T>>>,
// }
//
// impl<T> LinkedList<T> {
//     pub fn new() -> (res: Self)
//         ensures
//             res@ == Seq::<T>::empty(),
//     {
//         LinkedList { head: None }
//     }
//
//     pub fn push(&mut self, e: T)
//         ensures
//             final(self)@ =~= old(self)@.push(e),
//     {
//         Node::push_into_optional(&mut self.head, e);
//     }
//
//     pub fn get(&self, index: usize) -> (res: Option<&T>)
//         ensures
//             self@.len() > index ==> res == Some(&self@[index as int]),
//             self@.len() <= index ==> res == None,
//     {
//         Node::get_from_optional(&self.head, index)
//     }
//
//     pub closed spec fn as_seq(self) -> Seq<T> {
//         Node::<T>::optional_as_seq(self.head)
//     }
// }
//
// impl<T> View for LinkedList<T> {
//     type V = Seq<T>;
//
//     open spec fn view(&self) -> Self::V {
//         self.as_seq()
//     }
// }
//
// impl<T> Node<T> {
//     fn push_into_optional(node: &mut Option<Box<Node<T>>>, e: T)
//         ensures
//             final(node).is_some(),
//             old(node).is_none() ==> Node::<T>::optional_as_seq(*final(node)) =~= Node::<
//                 T,
//             >::optional_as_seq(*old(node)).push(e),
//             old(node).is_some() ==> Node::<T>::optional_as_seq(*final(node)) =~= Node::<
//                 T,
//             >::optional_as_seq(*old(node)).push(e),
//         decreases *old(node),
//     {
//         if node.is_none() {
//             *node = Some(Box::new(Node { e, next: None }));
//             proof {
//                 assert(*node is Some);
//                 assert(node.unwrap().e == e);
//                 let s = Node::<T>::optional_as_seq(*node);
//                 assert(s.len() > 0);
//                 assert(s.contains(e));
//             }
//         } else {
//             let mut tmp = None;
//             std::mem::swap(&mut tmp, node);
//             let mut boxed_node = tmp.unwrap();
//
//             (&mut *boxed_node).push(e);
//
//             *node = Some(boxed_node);
//         }
//     }
//
//     fn push(&mut self, e: T)
//         ensures
//             final(self).as_seq() =~= old(self).as_seq().push(e),
//         decreases *old(self),
//     {
//         Self::push_into_optional(&mut self.next, e);
//         assert(Node::<T>::optional_as_seq(self.next).contains(e));
//     }
//
//     fn get_from_optional(node: &Option<Box<Node<T>>>, current: usize) -> (res: Option<&T>)
//         ensures
//             *node is None ==> res is None,
//             *node is Some && current == 0 ==> res == Some(&node.unwrap().e),
//             *node is Some && current > 0 && current < Node::<T>::optional_as_seq(*node).len()
//                 ==> res == Some(&Node::<T>::optional_as_seq(*node)[current as int]),
//             *node is Some && current > 0 && current >= Node::<T>::optional_as_seq(*node).len()
//                 ==> res == None,
//         decreases current,
//     {
//         match node {
//             None => None,
//             Some(n) => {
//                 proof {
//                     test(*node);
//                 }
//                 if current == 0 {
//                     Some(&n.e)
//                 } else {
//                     Self::get_from_optional(&n.next, current - 1)
//                 }
//             },
//         }
//     }
//
//     spec fn optional_as_seq(node_opt: Option<Box<Node<T>>>) -> Seq<T>
//         decreases node_opt,
//     {
//         match node_opt {
//             None => Seq::empty(),
//             Some(node) => node.as_seq(),
//         }
//     }
//
//     pub closed spec fn as_seq(self) -> Seq<T>
//         decreases self,
//     {
//         let mut seq = Seq::empty();
//         seq.push(self.e);
//         seq.add(Node::<T>::optional_as_seq(self.next));
//         seq
//     }
// }
//
// proof fn test<T>(node: Option<Box<Node<T>>>)
//     requires
//         node is Some,
//     ensures
//         node is Some && node.unwrap().next is None ==> Node::<T>::optional_as_seq(node).len() == 1,
// {
//     let e = node.unwrap().e;
//
//     let s = Seq::empty();
//     s.push(e);
//     assert(s.len() == 1);
//
//     let seq = Node::<T>::optional_as_seq(node);
//
//     assert(s =~= seq);
// }
//
// fn main() {
//     let mut list = LinkedList::new();
//     list.push(5);
// }

// https://rosettacode.org/wiki/Palindrome_detection
// fn is_palindrome(string: &str) -> (res: bool)
//     ensures
//         res == forall|i: int|
//             0 <= i <= (string@.len() / 2) && #[trigger] string@[i]
//                 == #[trigger] string@[string@.len() - 1 - i],
// {
//     let len = string.len();
//     let chars = string.chars().collect::<Vec<char>>();
//     if len <= 1 {
//         return true;
//     }
//
//     let half_len = len / 2;
//
//     let mut i: usize = 0;
//     while i < half_len
//         invariant
//             0 <= i <= half_len + 1 <= len,
//             forall|j: int|
//                 0 <= j < i ==> #[trigger] string@[j] == #[trigger] string@[len - 1 - j],
//         decreases half_len - i,
//     {
//         assert(0 <= len - 1 - i < len);
//         assert(len == string@.len());
//         if chars[i] != chars[len - 1 - i] {
//             return false;
//         }
//         i += 1;
//     }
//
//     true
// }
//
// fn main() {
//     let c = is_palindrome("abba");
//     assert(c);
// }
// https://rosettacode.org/wiki/Ray-casting_algorithm
// const _EPS: f64 = 0.00001;
//
// const _MIN: f64 = f64::MIN_POSITIVE;
//
// const _MAX: f64 = f64::MAX;
//
// #[derive(Clone)]
// struct Point {
//     x: f64,
//     y: f64,
// }
//
// #[derive(Clone)]
// struct Edge {
//     pt1: Point,
//     pt2: Point,
// }
//
// impl Edge {
//     fn new(pt1: (f64, f64), pt2: (f64, f64)) -> Edge {
//         Edge { pt1: Point { x: pt1.0, y: pt1.1 }, pt2: Point { x: pt2.0, y: pt2.1 } }
//     }
// }
//
// struct Polygon {
//     edges: Vec<
//         Edge,
//     >,  // Polygon has to be created with counter-clockwise coordinates
// }
// fn pt_in_polygon(pt: &Point, poly: &Polygon) -> bool {
//     let mut count = 0;
//     for edge in &poly.edges {
//         if ray_intersect_seg(pt, edge) {
//             count += 1;
//         }
//     }
//
//     count % 2 == 1
// }
//
// fn ray_intersect_seg(p: &Point, edge: &Edge) -> (res: bool)
//     requires
//         p.x - edge.pt1.x >= f64::MIN,
//         p.x - edge.pt2.x >= f64::MIN,
//         p.y - edge.pt1.y >= f64::MIN,
//         p.y - edge.pt2.y >= f64::MIN,
//         edge.pt1.x - p.x >= f64::MIN,
//         edge.pt2.x - p.x >= f64::MIN,
//         edge.pt1.y - p.y >= f64::MIN,
//         edge.pt2.y - p.y >= f64::MIN,
//     ensures
//         (p.x > edge.pt1.x.max(edge.pt2.x)) ==> !res,
//         (p.y > edge.pt1.y.max(edge.pt2.y)) ==> !res,
//         (p.y < edge.pt1.y.min(edge.pt2.y)) ==> !res,
//         (p.x < edge.pt1.x.min(edge.pt2.x)) ==> res,
//
//         forall|i: int| edge.pt1.y.min(edge.pt2.y) <= i <= edge.pt1.y.max(edge.pt2.y)
//             && i ==
//
//
//         (p.x < edge.pt1.x.max(edge.pt2.x) && p.y <= edge.pt1.y.max(edge.pt2.y) && p.x
//             >= edge.pt1.x.max(edge.pt2.x)) ==> res == (),
// {
//     let mut pt = p.clone();
//     let (mut a, mut b): (&Point, &Point) = (&edge.pt1, &edge.pt2);
//     if a.y > b.y {
//         std::mem::swap(&mut a, &mut b);
//     }
//     // if pt.y == a.y || pt.y == b.y {
//     //     pt.y += _EPS;
//     // }
//
//     if (pt.y > b.y || pt.y < a.y) || pt.x > a.x.max(b.x) {
//         false
//     } else if pt.x < a.x.min(b.x) {
//         true
//     } else {
//         let m_red = if a.x != b.x {
//             (b.y - a.y) / (b.x - a.x)
//         } else {
//             f64::INFINITY
//         };
//         let m_blue = if a.x != pt.x {
//             (pt.y - a.y) / (pt.x - a.x)
//         } else {
//             f64::INFINITY
//         };
//         m_blue >= m_red
//     }
// }
// #[test]
// fn main() {
//     let p = |x, y| Point { x, y };
//     let testpoints = [
//         p(5.0, 5.0),
//         p(5.0, 8.0),
//         p(-10.0, 5.0),
//         p(0.0, 5.0),
//         p(10.0, 5.0),
//         p(8.0, 5.0),
//         p(10.0, 10.0),
//     ];
//     let poly_square = Polygon {
//         edges: vec![
//             Edge::new((0.0, 0.0), (10.0, 0.0)),
//             Edge::new((10.0, 0.0), (10.0, 10.0)),
//             Edge::new((10.0, 10.0), (0.0, 10.0)),
//             Edge::new((0.0, 10.0), (0.0, 0.0)),
//         ],
//     };
//     let poly_square_hole = Polygon {
//         edges: vec![
//             Edge::new((0.0, 0.0), (10.0, 0.0)),
//             Edge::new((10.0, 0.0), (10.0, 10.0)),
//             Edge::new((10.0, 10.0), (0.0, 10.0)),
//             Edge::new((0.0, 10.0), (0.0, 0.0)),
//             Edge::new((2.5, 2.5), (7.5, 2.5)),
//             Edge::new((7.5, 2.5), (7.5, 7.5)),
//             Edge::new((7.5, 7.5), (2.5, 7.5)),
//             Edge::new((2.5, 7.5), (2.5, 2.5)),
//         ],
//     };
//     let poly_strange = Polygon {
//         edges: vec![
//             Edge::new((0.0, 0.0), (2.5, 2.5)),
//             Edge::new((2.5, 2.5), (0.0, 10.0)),
//             Edge::new((0.0, 10.0), (2.5, 7.5)),
//             Edge::new((2.5, 7.5), (7.5, 7.5)),
//             Edge::new((7.5, 7.5), (10.0, 10.0)),
//             Edge::new((10.0, 10.0), (10.0, 0.0)),
//             Edge::new((10.0, 0.0), (2.5, 2.5)),
//         ],
//     };
//     let poly_hexagon = Polygon {
//         edges: vec![
//             Edge::new((3.0, 0.0), (7.0, 0.0)),
//             Edge::new((7.0, 0.0), (10.0, 5.0)),
//             Edge::new((10.0, 5.0), (7.0, 10.0)),
//             Edge::new((7.0, 10.0), (3.0, 10.0)),
//             Edge::new((3.0, 10.0), (0.0, 5.0)),
//             Edge::new((0.0, 5.0), (3.0, 0.0)),
//         ],
//     };
//     print!("\nSquare :");
//     for pt in &testpoints {
//         print!(" {:?}", pt_in_polygon(pt, &poly_square));
//     }
//     print!("\nSquare with hole:");
//     for pt in &testpoints {
//         print!(" {:?}", pt_in_polygon(pt, &poly_square_hole));
//     }
//     print!("\nStrange polygon :");
//     for pt in &testpoints {
//         print!(" {:?}", pt_in_polygon(pt, &poly_strange));
//     }
//     print!("\nHexagon :");
//     for pt in &testpoints {
//         print!(" {:?}", pt_in_polygon(pt, &poly_hexagon));
//     }
//     println!();
// }
} // verus!
use vstd::prelude::*;
