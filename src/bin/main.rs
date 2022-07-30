
fn main() -> Result<(), String> {

    #[cfg(feature="gdk4")]
    #[cfg(feature="gdk-pixbuf")]
    #[cfg(feature="cairo-rs")]
    {

        use std::io;
        // use std::io::Read;

        let mut model_str = String::new();
        // let panel : Option<papyri::render::Panel> = None;

        // Read input lines until a plot can be parsed. Perhaps issue a
        // timeout if no data is sent and the current input cannot be successfully parsed.
        //loop {
        let mut new_line = String::new();
        io::stdin().read_line(&mut new_line).unwrap();
        model_str += &new_line;
        //if let Ok(mut panel) = papyri::render::Panel::new_from_json(&model_str[..]) {
        let mut panel = papyri::render::Panel::new_from_json(&model_str[..]).map_err(|e| format!("{}",e) )?;

        //let svg = panel.svg().map_err(|e| format!("{}",e) )?;
        // println!("{}", svg);

        let html = panel.html_img_tag().map_err(|e| format!("{}",e) )?;
        println!("{}", html);

        return Ok(());
        // }
        // }
    }

    #[cfg(not(feature="gdk4"))]
    #[cfg(not(feature="gdk-pixbuf"))]
    #[cfg(not(feature="cairo-rs"))]
    Err(format!("Crate not compiled with features 'gdk4, gdk-pixbuf or cairo-rs'"))
}


