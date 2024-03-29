use nalgebra::DVector;
// use num_complex::Complex64;

/// Calculate Jenson-Shannon Divergence
pub fn jenson_shannon_divergence( p1: Vec<f64>, p2: Vec<f64>) -> f64{
    let p1 = DVector::from_vec(p1);
    let p2 = DVector::from_vec(p2);
    let mixed = 0.5*(&p1+&p2);
    return 0.5*(kl_divergence(p1,mixed.clone())+ kl_divergence(p2, mixed.clone()));
 
 }
 
 /// Calculate KL-divergence
pub fn kl_divergence( p: DVector<f64>, q: DVector<f64>)-> f64 {
    let mut temp = p.clone();
    for x in 0..temp.len() {
       temp[x] = - p[x]*f64::log2( q[x]/p[x]);
    }
    return temp.sum();
 }
 