use std::collections::HashMap;

fn euclidean_distance(v1: &[f32], v2: &[f32]) -> f32 {
    v1.iter()
        .zip(v2.iter())
        .map(|(x1, x2)| (x1 - x2).powi(2))
        .sum::<f32>()
        .sqrt()
}

fn find_closest_neighbors(target: &[f32], vectors: Vec<Vec<f32>>) -> Vec<(usize, f32)> {
    let mut distances: Vec<(usize, f32)> = vectors.iter()
        .enumerate()
        .map(|(i, v)| (i, euclidean_distance(target, v)))
        .collect();

    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    distances
}

pub fn find_closest(target: &[f32], vectors: &HashMap<String, Vec<f32>>) -> Vec<(String, f32)> {
    let mut distances: Vec<(String, f32)> = vectors.iter()
        .map(|(k, v)| (k.clone(), euclidean_distance(target, v)))
        .collect();

    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    distances
}