use pi::{IntoPI, PI};

pub fn p<T>(v: Vec<T>) -> impl PI<Item = T>
where
    // T: Send,
{
    v.into_pi()
}
