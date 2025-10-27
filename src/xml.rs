use crate::errors::KPSError;
use quick_xml::events::Event;
use quick_xml::Reader;

pub fn extract_first_tag_text(doc: &str, tag_suffix: &str) -> Result<Option<String>, KPSError> {
    let mut reader = Reader::from_str(doc);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut in_tag = false;
    let mut out = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if name.to_lowercase().ends_with(&tag_suffix.to_lowercase()) {
                    in_tag = true;
                }
            }
            Ok(Event::Text(e)) => {
                if in_tag {
                    out.push_str(&e.unescape().unwrap_or_default());
                }
            }
            Ok(Event::End(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if name.to_lowercase().ends_with(&tag_suffix.to_lowercase()) {
                    return Ok(Some(out));
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(KPSError::Parse(format!("XML parse error: {}", e))),
            _ => {}
        }
        buf.clear();
    }

    Ok(None)
}
