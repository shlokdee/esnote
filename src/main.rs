enum BlockType{
    Paragraph,
    //add more block types later (e.g. heading, levels for heading, etc)
}

struct Block{
    block_type:BlockType,
    content: String,
}

struct Page{
    title: String,
    blocks: Vec<Block>,
    children: Vec<Page>,
}

impl Block{
    fn new(text:String) -> Block {
        Block {
            block_type: BlockType::Paragraph,
            content: text,
        }
    }
}

impl Page{
    fn new(title:String) ->Page{
        Page{
            title: title,
            blocks: Vec::new(),
            children: Vec::new(),
        }
    }

    fn addblock(&mut self, newblock:Block){
       self. blocks.push(newblock);
    }
}



use eframe::egui;


struct EsNote{
    test:String,
}



impl eframe::App for EsNote{
    fn ui(&mut self, ui: &mut egui::Ui, _frame:&mut eframe::Frame){
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("test gui!")
        });
    }
}


fn main() -> eframe::Result<()>{
    /* before boilerplate code
    let mut firstpage=Page::new(String::from("epic page!"));
    let firstblock=Block::new(String::from("heres some block text"));
    firstpage.addblock(firstblock);
    */

    let esnote=EsNote{
        test: String::from("this is a test message"),
    };

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Es Note",
        native_options,
        Box::new(|_cc| Ok(Box::new(esnote)))
    )


}

