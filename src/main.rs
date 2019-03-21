#![feature(fn_traits, unboxed_closures)]

#[derive(Copy, Clone)]
pub struct CurryB<A, F> {
    a: A,
    func: F,
}

impl<A, B, C, F> FnOnce<(B, )> for CurryB<A, F>
    where F: Fn((A, B)) -> C {
    type Output = C;

    extern "rust-call" fn call_once(self, b: (B, )) -> Self::Output {
        (self.func)((self.a, b.0))
    }
}



fn curry<'a, 'b, F, A: 'a, B: 'b, C>(func: F) -> impl Fn(&'a A) -> CurryB<&'a A, F>
    where F: Fn((&'a A, &'b B)) -> C + Clone + 'a {
    move |a: &'a A| {
        CurryB { a, func: func.clone() }
    }
}

fn main() {
    let sum = |(a, b)| a + b;
    let sum_1 = (curry(sum))(&1);
    let sum_2 = (curry(sum))(&2);
    println!("sum: {:?}", sum_1(&2));
    println!("sum: {:?}", sum_1(&3));
    println!("sum: {:?}", sum_2(&3));
    println!("sum: {:?}", sum_2(&4));
}
