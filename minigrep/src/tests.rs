use super::*;

#[test]
fn rickroll_good() {
    let query = "good";
    let contents = "\
Never gonna make you cry
Never gonna say goodbye
Never gonna tell a lie and hurt you";

    assert_eq!(vec!["Never gonna say goodbye"], search(query, contents));
}

#[test]
fn ignore_case_good() {
    let query = "GoOd";
    let contents = "\
Never gonna make you cry
Never gonna say goodbye
Never gonna tell a lie and hurt you";

    assert_eq!(vec!["Never gonna say goodbye"], search_ignore_case(query, contents));

}
