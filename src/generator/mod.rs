pub mod er;

#[cfg(test)]
mod tests {
    use super::er::generate_erdos_renyi;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn er_generator_format() {
        let mut rng = StdRng::seed_from_u64(7);
        let mut out = Vec::new();
        generate_erdos_renyi(4, 5, &mut rng, &mut out).unwrap();
        let s = String::from_utf8(out).unwrap();
        for line in s.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            assert_eq!(parts.len(), 2);
            let u: u32 = parts[0].parse().unwrap();
            let v: u32 = parts[1].parse().unwrap();
            assert!(u < 4);
            assert!(v < 4);
        }
    }
}
