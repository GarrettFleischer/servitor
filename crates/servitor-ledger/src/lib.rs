mod error;
mod ids;
mod money;

pub fn workspace_boots() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase0_workspace_compiles() {
        assert!(workspace_boots());
    }
}
