use std::collections::HashMap;

fn euclidean_distance(v1: &[f32], v2: &[f32]) -> f32 {
    v1.iter()
        .zip(v2.iter())
        .map(|(x1, x2)| (x1 - x2).powi(2))
        .sum::<f32>()
        .sqrt()
}

pub fn find_closest(target: &[f32], vectors: &HashMap<String, Vec<f32>>) -> Vec<(String, f32)> {
    let mut distances: Vec<(String, f32)> = vectors.iter()
        .map(|(k, v)| (k.clone(), euclidean_distance(target, v)))
        .collect();

    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    distances
}

pub fn _find_most_similar(target: &[f32], vectors: &HashMap<String, Vec<f32>>) -> Vec<(String, f32)> {
    let mut similarities: Vec<(String, f32)> = vectors.iter()
        .map(|(filename, vector)| (filename.clone(), _cosine_similarity(target, vector)))
        .collect();

    // Сортируем по убыванию схожести
    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    similarities
}
fn _cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
    let dot_product: f32 = v1.iter().zip(v2.iter()).map(|(x1, x2)| x1 * x2).sum();
    let magnitude_v1 = v1.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
    let magnitude_v2 = v2.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();

    if magnitude_v1 == 0.0 || magnitude_v2 == 0.0 {
        // Избегаем деления на ноль
        return 0.0;
    }

    dot_product / (magnitude_v1 * magnitude_v2)
}
