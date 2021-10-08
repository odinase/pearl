use argmax::Argmax;

pub fn logsumexp(v: &[f64]) -> f64 {
    let (_, b) = v.iter().argmax().unwrap();
    b + v.iter().map(|&v| (v - b).exp()).sum::<f64>().ln()
}