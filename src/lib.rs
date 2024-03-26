pub mod bpmf;
pub mod bpmf_chars;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests;
