type StateId = usize;

pub trait Dfa<Alphabet>
where
    Alphabet: Eq,
{
    fn initial_state(&self) -> StateId;

    fn is_accept_state(&self, state: StateId) -> bool;

    fn next_state(&self, current_state: StateId, input: &Alphabet) -> StateId;

    fn accepts(&self, input: &[Alphabet]) -> bool {
        let mut current_state = self.initial_state();
        for c in input {
            current_state = self.next_state(current_state, c);
        }
        if self.is_accept_state(current_state) {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Recognize001AsSubstr;

    impl Dfa<u8> for Recognize001AsSubstr {
        fn initial_state(&self) -> StateId {
            0
        }

        fn is_accept_state(&self, state: StateId) -> bool {
            state == 3
        }

        fn next_state(&self, current_state: StateId, input: &u8) -> StateId {
            match (current_state, input) {
                (0, 0) => 1,
                (0, 1) => 0,
                (1, 0) => 2,
                (1, 1) => 0,
                (2, 0) => 2,
                (2, 1) => 3,
                (3, _) => 3,
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn recognize_001_as_substr() {
        let input = [1, 0, 1, 1, 0, 0, 1, 0, 1];
        let dfa = Recognize001AsSubstr;
        assert!(dfa.accepts(&input))
    }
}
