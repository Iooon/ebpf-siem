use super::*;

#[test]
fn build_command() {
    let command = Command {
        userid: 1,
        command: "test".into(),
    };
    assert_eq!(command.userid, 1);
}
