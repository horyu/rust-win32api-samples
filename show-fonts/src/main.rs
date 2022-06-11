use windows::{core::*, Win32::Graphics::DirectWrite::*};

fn main() -> Result<()> {
    unsafe {
        let writefactory: IDWriteFactory7 = std::mem::transmute(DWriteCreateFactory(
            DWRITE_FACTORY_TYPE_SHARED,
            &IDWriteFactory::IID,
        )?);
        let mut fontcollectionopt = None;
        writefactory.GetSystemFontCollection(&mut fontcollectionopt, false)?;
        let fontcollection = fontcollectionopt.unwrap();
        for i in 0..fontcollection.GetFontFamilyCount() {
            let fontfamily = fontcollection.GetFontFamily(i)?;
            let familynames = fontfamily.GetFamilyNames()?;
            let mut familynamestrings = vec![];
            for j in 0..familynames.GetCount() {
                let length = familynames.GetStringLength(j)? as usize;
                let mut stringbuffer = vec![0; length + 1];
                familynames.GetString(j, &mut stringbuffer)?;
                let familynamestring = String::from_utf16(&stringbuffer[..(length)]).unwrap();
                familynamestrings.push(familynamestring);
            }
            println!("{i}: {}", familynamestrings.join(" | "));

            for j in 0..fontfamily.GetFontCount() {
                let font = fontfamily.GetFont(j)?;
                let facename = font.GetFaceNames()?;
                let mut localenames = vec![];
                for k in 0..facename.GetCount() {
                    let length = facename.GetStringLength(k)? as usize;
                    let mut localename = vec![0u16; length + 1];
                    facename.GetString(k, &mut localename)?;
                    localenames.push(String::from_utf16(&localename[..(length)]).unwrap());
                }
                println!(
                    "  {j}({},{},{}): {}",
                    font.GetStretch().0,
                    font.GetStyle().0,
                    font.GetWeight().0,
                    localenames.join(" | ")
                );
            }
        }
    }
    Ok(())
}
