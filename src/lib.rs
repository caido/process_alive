mod pid;
pub use self::pid::*;
mod state;
pub use self::state::*;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use self::unix::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_alive() {
        let mut child = Command::new("sleep").arg("5").spawn().unwrap();
        let pid = child.id().into();
        let state = state(pid);
        child.kill().unwrap();
        child.wait().unwrap();
        assert_eq!(State::Alive, state);
    }

    #[test]
    fn test_dead() {
        let mut child = Command::new("sleep").arg("5").spawn().unwrap();
        let pid = child.id().into();
        child.kill().unwrap();
        child.wait().unwrap();
        let state = state(pid);
        assert_eq!(State::Dead, state);
    }
}
