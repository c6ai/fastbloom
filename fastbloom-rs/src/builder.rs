use crate::filter::BloomFilter;

#[derive(Clone)]
#[derive(Debug)]
pub struct FilterBuilder {
    expected_elements: u64,
    false_positive_probability: f64,
    pub(crate) size: u64,
    pub(crate) hashes: u32,
    done: bool,
}

pub(crate) const SUFFIX: u64 = 0b0001_1111;
pub(crate) const MASK: u64 = 0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11100000;

/// Calculates the optimal size `m` of the bloom filter in bits given `n` (expected
/// number of elements in bloom filter) and `p` (tolerable false positive rate).
#[inline]
fn optimal_m(n: u64, p: f64) -> u64 {
    let fact = -(n as f64) * p.ln();
    let div = 2f64.ln().powi(2);
    let m: f64 = fact / div;
    let mut m = m.ceil() as u64;
    if (m & SUFFIX) != 0 {
        m = (m & MASK) + SUFFIX + 1;
    };
    m
}

/// Calculates the optimal `hashes` (number of hash function) given `n` (expected number of
/// elements in bloom filter) and `m` (size of bloom filter in bits).
#[inline]
fn optimal_k(n: u64, m: u64) -> u32 {
    let k: f64 = (m as f64 * 2f64.ln()) / n as f64;
    k.ceil() as u32
}

impl FilterBuilder {
    /// Constructs a new Bloom Filter Builder by specifying the expected size of the filter and the
    /// tolerable false positive probability. The size of the BLoom filter in in bits and the
    /// optimal number of hash functions will be inferred from this.
    pub fn new(expected_elements: u64, false_positive_probability: f64) -> Self {
        FilterBuilder {
            expected_elements,
            false_positive_probability,
            size: 0,
            hashes: 0,
            done: false,
        }
    }

    pub(crate) fn from_size_and_hashes(size: u64, hashes: u32) -> Self {
        todo!()
    }

    fn expected_elements(&mut self, expected_elements: u64) {
        assert!(expected_elements > 0, "expected_elements must larger than 0!");
        self.expected_elements = expected_elements;
    }

    fn false_positive_probability(&mut self, false_positive_probability: f64) {
        assert!(false_positive_probability < 1.0 && false_positive_probability > 0.0,
                "false_positive_probability must between (0.0, 1.0)!");
        self.false_positive_probability = false_positive_probability;
    }

    fn size(&mut self, size: u64) {
        self.size = size;
    }


    /// todo from_size_and_hashes
    pub(crate) fn complete(&mut self) {
        if !self.done {
            if self.size == 0 {
                self.size = optimal_m(self.expected_elements, self.false_positive_probability);
                self.hashes = optimal_k(self.expected_elements, self.size);
            }
            self.done = true;
        }
    }

    pub fn build_bloom_filter(&mut self) -> BloomFilter {
        self.complete();
        BloomFilter::new(self.clone())
    }

    ///
    pub(crate) fn is_compatible_to(&self, other: &FilterBuilder) -> bool {
        self.size == other.size && self.hashes == other.hashes
    }
}

#[test]
fn optimal_test() {
    let m = optimal_m(216553, 0.01);
    let k = optimal_k(216553, 2075680);
    println!("{}", SUFFIX);
    println!("{m} {k}");
    assert_eq!(m, 2075680);
    assert_eq!(k, 7)
}

#[test]
fn builder_test() {
    let mut bloom = FilterBuilder::new(100_000_000, 0.01)
        .build_bloom_filter();
    bloom.add(b"helloworld");
    assert_eq!(bloom.contains(b"helloworld"), true);
    assert_eq!(bloom.contains(b"helloworld!"), false);
}