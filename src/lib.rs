use bio::{alphabets::{Alphabet, self}, data_structures::{suffix_array::suffix_array, bwt::{bwt, less, Occ}, fmindex::{FMIndex, BackwardSearchResult, FMIndexable}}, alignment::distance::{simd, levenshtein}};

fn build_fm_index<'a>(text: &'a [u8], alphabet: &'a Alphabet) -> (FMIndex<Vec<u8>, Vec<usize>, Occ>, Vec<usize>) {
    let sa = suffix_array(text);
    let bwt = bwt(text, &sa);

    let less = less(&bwt, alphabet);
    let occ = Occ::new(&bwt, 3, alphabet);

    (FMIndex::new(bwt, less, occ), sa)
}

#[test]
fn test_fm_index_search() {
    let text = b"ACAGCTCGATCGGTA$";
    let pattern = b"ATCG";

    let alphabet = alphabets::dna::alphabet();

    let (fmindex, sa) = build_fm_index(text, &alphabet);
    let interval = fmindex.backward_search(pattern.iter());
    let mut partial_match_len = 0;

    let positions = match interval {
        BackwardSearchResult::Complete(saint) => saint.occ(&sa),
        BackwardSearchResult::Partial(saint, l) => {
            partial_match_len = l;
            saint.occ(&sa)
        }
        BackwardSearchResult::Absent => Vec::new(),
    };
    

    println!("{:?}", positions);
    println!("{}", partial_match_len);
}

#[test]
fn test_fm_index_search_partial() {
    let text = b"ACAGCTCGTCGGTA$";
    let pattern = b"ATCG";

    let alphabet = alphabets::dna::alphabet();

    let (fmindex, sa) = build_fm_index(text, &alphabet);
    let interval = fmindex.backward_search(pattern.iter());
    let mut partial_match_len = 0;

    let positions = match interval {
        BackwardSearchResult::Complete(saint) => saint.occ(&sa),
        BackwardSearchResult::Partial(saint, l) => {
            partial_match_len = l;
            saint.occ(&sa)
        }
        BackwardSearchResult::Absent => Vec::new(),
    };
    

    println!("{:?}", positions);
    println!("{}", partial_match_len);
}

#[test]
fn test_alignment_levenshtein_distance() {
    let a = b"ATCG";
    let b = b"CTCGATC";

    let ldist = levenshtein(a, b);

    assert_eq!(ldist, 4);
}

#[test]
fn test_alignment_simd_levenshtein_distance() {
    let a = b"ATCG";
    let b = b"CTCGATC";

    let ldist = simd::levenshtein(a, b);

    assert_eq!(ldist, 4);
}

#[test]
fn test_alignment_simd_levenshtein_distance_optimized() {
    // Assume that these are long strings :P
    let a = b"ATCG";
    let b = b"CTCGATC";

    let ldist = simd::bounded_levenshtein(a, b, 5);

    let val = match ldist {
        Some(val) => val,
        None => levenshtein(a, b),
    };

    assert_eq!(val, 4);
}