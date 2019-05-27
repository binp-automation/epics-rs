use super::{
    AiRecord, AoRecord,
//    BiRecord, BoRecord,
//    LonginRecord, LongoutRecord,
//    StringinRecord, StringoutRecord,
};


/// Reference to any record
pub enum AnyRecordRef<'a> {
    Ai(&'a mut AiRecord),
    Ao(&'a mut AoRecord),
    // Bi(BiRecord),
    // Bo(BoRecord),
    // Longin(LonginRecord),
    // Longout(LongoutRecord),
    // Stringin(StringinRecord),
    // Stringout(StringoutRecord),
}
