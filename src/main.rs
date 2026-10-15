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
    pages: Vec<Page>,
    page_idx: usize,
}



impl eframe::App for EsNote{
    fn ui(&mut self, ui: &mut egui::Ui, _frame:&mut eframe::Frame){

        egui::Panel::left("sidebar_panel").show(ui,|ui|{
            ui.heading("pages");
            ui.separator();

            for (index,page) in self.pages.iter().enumerate(){
                if ui.button(&page.title).clicked(){
                    self.page_idx=index;
                }
            }
        });



        egui::CentralPanel::default().show(ui, |ui| {
            let current_page=&self.pages[self.page_idx];

            ui.heading(&current_page.title);
            ui.separator();
            
            for block in &current_page.blocks{
                ui.label(&block.content);
            }
            
        });

        
    }
}


fn main() -> eframe::Result<()>{

    let mut firstpage=Page::new(String::from("epic page!"));
    let firstblock=Block::new(String::from("heres some block text"));
    firstpage.addblock(firstblock);

    let mut secondpage=Page::new(String::from("title2!"));

    let mut pagevec=Vec::new();
    pagevec.push(firstpage);
    pagevec.push(secondpage);
    

    let esnote=EsNote{
        pages:pagevec,
        page_idx:0,
    };

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "EsNote",
        native_options,
        Box::new(|_cc| Ok(Box::new(esnote)))
    )


}

