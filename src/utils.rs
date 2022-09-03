use argmax::Argmax;

pub fn logsumexp(v: &[f64]) -> f64 {
    let (_, b) = v.iter().argmax().unwrap();
    b + v.iter().map(|&v| (v - b).exp()).sum::<f64>().ln()
}

// Returns Option as p and q might be different sizes. Stupid?
pub fn kl_divergence(p: &[f64], q: &[f64]) -> Option<f64> {
    if p.len() != q.len() {
        return None;
    } else {
        Some(
            p.iter().zip(q.iter())
            .map(
                |(p, q)| p * (p / q).ln()
            )
            .sum()
        )
    }
}