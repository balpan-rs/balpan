// ref: https://www.sspilsbury.com/2017-09-23-explaining-boyer-moore/
// ref: https://github.com/peterjoel/needle/blob/master/src/skip_search.rs

pub struct BoyerMooreSearch<'a, T> {
    pattern: &'a [T],
    bad_character_table: [usize; 256],
    good_suffixes_table: Vec<usize>,
}

impl<'a, T> BoyerMooreSearch<'a, T>
where
    T: Copy + PartialEq + Into<usize>,
{
    /// Create new Boyer-Moore Search object with given pattern.
    ///
    /// ### Example
    ///
    /// Basic usage:
    ///
    /// If you want to search a pattern ("abc" in this case) in a text,
    /// you can simply put it in the function as an argument.
    ///
    /// ```
    /// use balpan::commands::boyer_moore::BoyerMooreSearch;
    ///
    /// let searcher = BoyerMooreSearch::new(b"abc");
    /// ```
    pub fn new(pattern: &'a [T]) -> BoyerMooreSearch<T> {
        Self {
            pattern,
            bad_character_table: build_bad_chars_table(pattern),
            good_suffixes_table: build_suffixes_table(pattern),
        }
    }
}

/// `SearchIn` trait is define the interface which can iterate over the pattern in the text.
pub trait SearchIn<'a, H: ?Sized> {
    type Iter: Iterator<Item = usize>;

    fn find_in(&'a self, text: &'a H) -> Self::Iter;
    fn find_overlapping_in(&'a self, text: &'a H) -> Self::Iter;
    /// Find the first occurrence of the pattern within the given text.
    fn find_first_position(&'a self, text: &'a H) -> Option<usize> {
        self.find_in(text).next()
    }
}

impl<'a, T> SearchIn<'a, [T]> for BoyerMooreSearch<'a, T>
where
    T: Copy + PartialEq + Into<usize>,
{
    type Iter = BoyerMooreIter<'a, T>;
    /// Find all occurrences of the pattern within the given text,
    /// but only consider non-overlapping cases.
    ///
    /// `find_in` skips over the length of the pattern each time
    /// a match is found, so that overlapping occurrences are ignored.
    ///
    /// ### How it works:
    ///
    /// 1. Initialize the search at the beginning of the text.
    /// 2. Compare the pattern with the text at the current position.
    /// 3. If a match is found, yield the current position and skip forward by the parent's length (to ensure no overlaps).
    /// 4. If no match is found, apply the Boyer-Moore skipping rules (bad character and good suffix rules)
    /// to jump ahead and continue to search.
    /// 5. Repeat steps 2-4 until the end of the text is reached.
    ///
    /// ### Example
    ///
    /// Basic usage:
    ///
    /// ``` ignore
    /// use balpan::commands::boyer_moore::{BoyerMooreSearch, SearchIn};
    ///
    /// let searcher = BoyerMooreSearch::new(b"aba");
    /// let text = b"ababa";
    ///
    /// let result: Vec<usize> = searcher.find_in(text).collect();
    ///
    /// assert_eq!(vec![0], result);
    /// ```
    fn find_in(&'a self, text: &'a [T]) -> Self::Iter {
        BoyerMooreIter {
            searcher: self,
            text,
            pos: 0,
            overlap_match: false,
        }
    }
    /// Find all the overlapping occurrences of the pattern within given text, including the overlapping matches.
    /// Unlike the `find_in` method, which ony considers non-overlapping cases.
    /// by considering each position in the text as a starting point for the pattern.
    ///
    /// ### How it works:
    ///
    /// 1. Initialize the search at the beginning of the text.
    /// 2. Compare the pattern with the text at the current position.
    /// 3. If a match is found, yield the current position and move only one position forward
    /// (instead of skipping by the parent's length).
    /// 4. If no match is found, apply the Boyer-Moore skipping rules (bad character and good suffix rules)
    /// to jump ahead and continue to search.
    /// 5. Repeat steps 2-4 until the end of the text is reached.
    ///
    /// ### Example
    ///
    /// ``` ignore
    /// use balpan::commands::boyer_moore::{BoyerMooreSearch, SearchIn};
    ///
    /// let searcher = BoyerMooreSearch::new(b"aaba");
    /// let text = b"aabaabaaba";
    ///
    /// let result: Vec<usize> = searcher.find_overlapping_in(text).collect();
    ///
    /// assert_eq!(vec![0, 3, 6], result);
    /// ```
    ///
    /// The `find_overlapping_in` method would find matches at some positions,
    /// which means that the pattern "aaba" occurs at positions 0, 3, and 6 in the text.
    fn find_overlapping_in(&'a self, text: &'a [T]) -> Self::Iter {
        BoyerMooreIter {
            searcher: self,
            text,
            pos: 0,
            overlap_match: true,
        }
    }
}

pub struct BoyerMooreIter<'a, T> {
    searcher: &'a BoyerMooreSearch<'a, T>,
    text: &'a [T],
    pos: usize,
    overlap_match: bool,
}

impl<'a, T> Iterator for BoyerMooreIter<'a, T>
where
    T: Copy + PartialEq + Into<usize>,
{
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        find_from_position(&self.searcher, self.text, self.pos).map(|pos| {
            match self.overlap_match {
                true => self.pos = pos + 1,
                false => self.pos = pos + self.searcher.pattern.len(),
            }

            pos
        })
    }
}
/// `find_pending_character_index` method is looking for the occurrence of a specific character (pattern)
/// in a given slice of characters (`chars`).
///
/// If the character is found, the function returns the index of the found character - start index (`start`),
/// effectively returning the relative position of the found character within the slice starting from the start index (`start`).
///
/// If the character is not found, simply return 0.
///
/// ### How it works:
/// 1. Iterate through the slice of characters starting from the index 'start + 1'.
/// 2. Compare each character with the given pattern.
/// 3. If a match is found, return the relative position (i.e., the current index minus the start index).
/// 4. If no match is found, return 0.
///
/// ### Example
///
/// - chars: \['A', 'B', 'C', 'B', 'D'\]
/// - start: 1
/// - pattern: 'B'
///
///  Step 1: Start searching from index `start + 1` (i.e., 2):
///  
///     chars  A    B    C    B    D
///     index  0    1    2    3    4
///     start       ^
///
/// Step 2: Compare each character with the given pattern 'B':
///
///     chars  A    B    C    B    D
///     index  0    1    2    3    4
///     start       ^    ^    ^    ^
///     pattern          B    B    B
///
/// Step 3: Pattern 'B' found at index 3, relative position is 3 - 1 = 2.
///
///     chars  A    B    C    B    D
///     index  0    1    2    3    4
///     start       ^         ^
///     pattern          B    B
///     Result  2
///
/// ### Example
///
/// ```ignore
/// use balpan::commands::boyer_moore::find_pending_character_index;
///
/// let chars = vec!['A', 'B', 'C', 'B', 'D'];
/// let start = 1;
/// let pattern = &'B';
///
/// let result = find_pending_character_index(&chars, start, pattern);
///
/// assert_eq!(2, result);
/// ```
pub fn find_pending_character_index(chars: &[char], start: usize, pattern: &char) -> usize {
    for (i, item) in chars.iter().enumerate().skip(start + 1) {
        if item == pattern {
            return i - start;
        }
    }

    0
}
/// `build_bad_chars_table` method is building a table of bad characters, which is key part of the Boyer-Moore algorithm.
///
/// ### Description
///
/// This method pre-computes a table that allows the algorithm to skip sections of the text to be searched,
/// resulting in a lower number of overall character comparisons.
///
/// In other words, this method creates a table that helps the main search function
/// know how far to jump when a mismatch is found.
///
/// The table's size is usually 256 bytes, to cover all possible ASCII characters.
///
/// ### How it works:
///
/// For example, let's assume a pattern "GATC":
///
/// - pattern: "GATC"
/// - length: 4
///
/// Step 1: Initialize the table with the length of the pattern:
///
///     table   A B C D E F G ... T U V W X Y Z
///     value   4 4 4 4 4 4 4 ... 4 4 4 4 4 4 4
///
/// Step 2: Iterate through the pattern and update the table except the last character:
///
///     'G' is at index 0, distance to end - 1 = 4 - 0 - 1 = 3
///     'A' is at index 1, distance to end - 1 = 4 - 1 - 1 = 2
///     'T' is at index 2, distance to end - 1 = 4 - 2 - 1 = 1
///     'C' is the last character, skip
///
/// Step 3: Update the table with the calculated distances:
///
///     table   A B C D E F G ... T U V W X Y Z
///     value   2 4 4 4 4 4 3 ... 1 4 4 4 4 4 4
///
/// ### Conclusion
///
/// This table is used in the search process, allowing the BM search to skip over portions of
/// the text that do not contain possible matches, thereby reducing the number of comparisons.
pub fn build_bad_chars_table<T>(needle: &[T]) -> [usize; 256]
where
    T: Into<usize> + Copy,
{
    let mut table = [needle.len(); 256];
    for i in 0..needle.len() - 1 {
        let c: usize = needle[i].into();
        table[c] = needle.len() - i - 1;
    }

    table
}
/// `get_suffix_table` method computes the suffix table.
/// This table helps in defining how much to jump in case of a mismatch after some matches.
///
/// ### Description
///
/// This method computes a table where the entry at index `i` represents the length of
/// the largest suffix of the pattern ending at position `i` that is also a prefix of the pattern.
///
/// ### How it works:
///
/// Assume a pattern is "ABAB":
///
/// Step 1: Initialize the suffixes table with 0
///
///     table       A B A B
///     suffixes    0 0 0 0
///
/// Step 2: Start with suffix length 1. and check for each suffix it's a prefix of the pattern.
///
///     For suffix length `1`: `B` is not a prefix, continue
///
///     For suffix length `2`: `AB` is a prefix, update the entry
///
///                 table       A B A B
///                 suffixes    0 0 2 0
///
///     For suffix length `3`: `BAB` is not a prefix, continue
///
/// ### Conclusion
///
/// This table used to create the good suffix shift table, which tells the how far to go
/// in case of a mismatch. By understanding the structure of the pattern itself, the BM
/// can skip ahead more efficiently, by reduce the number of comparisons.
pub fn get_suffix_table<T: PartialEq>(pattern: &[T]) -> Vec<usize> {
    let len = pattern.len();
    let mut suffixes = vec![0; len];
    for suffix_len in 1..pattern.len() {
        let mut found_suffix = false;
        for i in (0..len - suffix_len).rev() {
            // either 0 or a previous match for a 1-smaller suffix
            if suffixes[i + suffix_len - 1] == suffix_len - 1
                && pattern[i] == pattern[len - suffix_len]
            {
                suffixes[i + suffix_len - 1] = suffix_len;
                found_suffix = true;
            }
        }

        if !found_suffix {
            break;
        }
    }

    suffixes
}
/// Builds the "good suffix table,"
/// which is an essential part of the Boyer-Moore algorithm's optimization.
///
/// It's used to determine how far to jump along the text when a mismatch occurs
/// in the pattern after some matches.
///
/// ### Description
///
/// This method takes the suffix table computed by `get_suffix_table`
/// and builds a table that directly tells the algorithm how far to jump
/// in case of a mismatch at a given position.
///
/// ### How it works:
///
/// 1. Initializes a table with the pattern's length minus one at all positions.
/// 2. Updates the table using the suffixes from get_suffix_table,
/// making sure that the jumps are optimized according to the pattern's internal structure.
/// 3. Specifically sets the last element of the table to 1,
/// as the jump should always be at least one character.
///
/// Using the pattern "ABAB" and assuming the suffix table as `[0, 0, 2, 0]`:
///
/// Step 1: Initialize the table with the length of the needle minus one (3)
///
///     pattern: A  B  A  B
///     needle:  3  3  3  3
///
/// Step 2: Iterate through the suffixes table and update the entries
///
///     - suffix length 2 at index 2, skip 2 positions
///         A  B  A  B
///         3  3  2  3
///
/// Step 3: Set the last entry to 1
///
///     A  B  A  B
///     3  3  2  1
pub fn build_suffixes_table<T: PartialEq>(pattern: &[T]) -> Vec<usize> {
    let suffixes = get_suffix_table(pattern);
    let len = pattern.len();
    let mut table = vec![len - 1; len];

    for (i, suffix_len) in suffixes.into_iter().enumerate() {
        let needle_index = len - suffix_len - 1;
        let skip = len - i - 1;
        if table[needle_index] > skip {
            table[needle_index] = skip;
        }
    }

    table[len - 1] = 1;
    table
}

pub trait SkipSearch<T> {
    fn skip_offset(&self, bad_char: T, pattern_pos: usize, text: &[T], text_pos: usize) -> usize;
    fn len(&self) -> usize;
    fn at(&self, index: usize) -> T;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub fn find_from_position<'a, T, U>(
    pattern: &'a U,
    text: &'a [T],
    mut position: usize,
) -> Option<usize>
where
    T: PartialEq + Copy + Into<usize>,
    U: SkipSearch<T>,
{
    if pattern.len() > text.len() {
        return None;
    }

    let max_position = text.len() - pattern.len();
    while position <= max_position {
        let mut pattern_pos = pattern.len() - 1;

        while text[position + pattern_pos] == pattern.at(pattern_pos) {
            if pattern_pos == 0 {
                return Some(position);
            }

            pattern_pos -= 1;
        }

        let bad_char = text[position + pattern.len() - 1];
        position += pattern.skip_offset(bad_char, pattern_pos, text, position);
    }

    None
}

impl<'a, T> SkipSearch<T> for &'a BoyerMooreSearch<'a, T>
where
    T: Copy + Into<usize>,
{
    fn skip_offset(&self, bad_char: T, pattern_pos: usize, _text: &[T], _text_pos: usize) -> usize {
        let bad_char_shift = self.bad_character_table[bad_char.into()];
        let good_suffix_shift = self.good_suffixes_table[pattern_pos];

        std::cmp::max(bad_char_shift, good_suffix_shift)
    }

    fn len(&self) -> usize {
        self.pattern.len()
    }

    fn at(&self, pos: usize) -> T {
        self.pattern[pos]
    }
}
