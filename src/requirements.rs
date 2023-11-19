use serde::Deserialize;

use crate::alias::Requirement;

// Likely doesn't Deserialize Correctly
#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(tag = "type", content = "args")]
#[serde(rename_all = "snake_case")]
pub enum ReqType {
    Or(Vec<ReqType>),
    // NODE
    And(Vec<ReqType>),
    // NODE
    HasItem(String),
    // Leaf
    Count(u8, String),
    // (Complex) Leaf
    CanAccess(String),
    // NODE
    Setting(String),
    // "Leaf" | Static, Part of the BitVec Header/World State Header Or Item State
    Macro(String, Box<ReqType>),
    // NODE Non-Special Case
    Not(String),
}

/// 0b1000101010010100
/// NODE
/// 1 OR MORE Bits
/// OR Node
///  (OR SPACE)   (Item Space)
/// 0b00000010 00000000000000
///
///  Example
/// {
///   "type": "or": "args": [
///    {  "type": "has_item": "args": "Power Bracelets" },
///    {  "type": "has_item": "args": "Bombs" }
/// ]
/// }
/// (EQ)
/// { "type": "macro": "args": "Can Lift Boulders" }
///
/// Items - String - Unique Randomization ID
/// World State - BitVec
/// Player gets Items -> State | Item (Maybe State + Item? Unsure to how handle count)
///
/// Power Bracelets = ID 1
/// Bombs = ID 2
/// Power Bracelets | Bombs = ID 8
///
/// Power Bracelets on a location have a "State" Id of 9, Bombs would be 10
///
/// 0b1000 (OR Statement)
///
/// 0b1001 PB
/// 0b1010 Bombs
/// 0b1011 PB | Bombs
///
/// 0 = (OR ^ State) & OR;
/// 0b1000 ^ 0b1010 = 0b0010
/// 0b1000 & 0b0010 = 0b0000
/// 0 = 0b0 Requirement has passed
///
/// (AND Statement)
/// 0b0011 PB & Bombs
///
/// (1 OR (2 AND 3))
/// Store 2 States within 1 BitVec designated by length. So BitVec.len() / MaxSpace = Check amount.
/// Faster than Vec<BitVec> ? - Benchmarking requirement :(
/// 0b0001
/// 0b0110
///
/// ((1 OR 2) AND 3)
/// 0b0101
/// 0b0110
///
/// How to Count?
/// (3 Small Keys)
/// - Location Flag or BitVec Header (Would have to be a constant in the World State BitVec as well)
///    - Disregard and do logic manually
/// (other_evaluation & handleCount(world, count_struct)
/// Counting is only an issue if there is a location/check that requires a subset of max.
///
/// "Ideally Linear"
///
/// Multiworld Exponential Problem :(
///
/// World Count where Multithreading is faster, but where is it?
/// Multithreading Strategy can change on world ranges
///
/// How to Provide this to the Tool
/// 2 Steps
/// Provide World and Macro (Slow and Lame)
/// Precompile World and Macro (Fast and Cool)
///


/// Applies `requirement_two` to `requirement_one` based on the `req_type`
///
/// # Notes
/// Currently the only value [`ReqType`] values are
/// - [`ReqType::Or`] ORs the two values
/// - [`ReqType::And`] ANDs the two values
/// - [`ReqType::Not`] NOTs `requirement_two` then ANDs it to `requirement_one`
/// - [`ReqType::None`] Returns `requirement_one`
fn combine_requirements(requirement_one: Requirement, requirement_two: Requirement, req_type: ReqType) -> Requirement {
    todo!()
    // return match req_type {
    //     ReqType::Or => requirement_one | requirement_two,
    //     ReqType::And => requirement_one & requirement_two,
    //     ReqType::Not => requirement_one & !requirement_two,
    //     ReqType::None => requirement_one,
    //     _ => {
    //         print!("Invalid Req Type Provided {}", stringify!(req_type));
    //         requirement_one
    //     }
    // }
}
// # https://docs.rs/jsonc-parser/0.23.0/jsonc_parser/

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn it_deserializes_req_type_from_json_correctly() {
        let data = r#"
        {
                        "type": "or", "args": [
                            { "type": "can_access", "args": "TOTGSecondStatueRoomNearStatue"}
                        ]
                    }"#;

        let got: ReqType = serde_json::from_str(data).unwrap();
        assert_eq!(got, ReqType::Or(vec![ReqType::CanAccess("TOTGSecondStatueRoomNearStatue".to_string())]));
    }

    #[test]
    fn count_req_type_should_deserialize_correctly() {
        let data = r#"
        { "type": "count", "args": [2, "TotGSmallKey"]}
        "#;
        let count_req: ReqType = serde_json::from_str(data).unwrap();
        assert_eq!(count_req, ReqType::Count(2, "TotGSmallKey".to_string()));
    }
}
