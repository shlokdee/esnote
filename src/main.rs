#[derive(PartialEq)]

enum BlockType{
    Paragraph,
    Heading1,
    Heading2,
    Heading3,
}

struct Block{
    block_type:BlockType,
    content: String,
    needs_focus:bool,
    is_editing:bool,
}

struct Page{
    title: String,
    blocks: Vec<Block>,
    block_idx: usize,
    children: Vec<Page>,


}

impl Block{
    fn new(text:String) -> Block {
        Block {
            block_type: BlockType::Paragraph,
            content: text,
            needs_focus:true,
            is_editing:true,
        }
    }
}

impl Page{
    fn new(title:String) ->Page{
        Page{
            title: title,
            blocks: Vec::new(),
            block_idx:0,
            children: Vec::new(),

        }
    }

    fn addblock(&mut self, newblock:Block, index:usize){
       self. blocks.insert(index,newblock);
    }

    fn markdown(&self) -> String{
        let mut output=format!("# {}\n\n", self.title);
        for block in &self.blocks{
            output.push_str(&format!("{}\n\n", block.content));
        }

        output
    }
}



use eframe::egui;
use egui::Vec2;


struct EsNote{
    pages: Vec<Page>,
    page_idx: usize,
    new_page_name: String,
    addtoggle:bool,
}



impl eframe::App for EsNote{
    fn ui(&mut self, ui: &mut egui::Ui, _frame:&mut eframe::Frame){

        


        egui::Panel::left("sidebar_panel").show(ui,|ui|{
            ui.heading("pages");
            ui.separator();

            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui|{
                for (index,page) in self.pages.iter().enumerate(){
                if ui.selectable_label(index==self.page_idx,&page.title).clicked(){
                    self.page_idx=index;

                }
                
            }});

            if ui.button("+").clicked(){
                self.addtoggle=true;
            }
            if self.addtoggle{
                ui.text_edit_singleline(&mut self.new_page_name);
                    let mut newpage=Page::new(String::from(&self.new_page_name));
                    let firstblock=Block::new(String::from(""));
                    newpage.addblock(firstblock,0);
                    if ui.button("add").clicked(){
                        self.pages.push(newpage);
                        self.page_idx=self.pages.len()-1;
                        self.addtoggle=false;
                    }
                    if ui.button("cancel").clicked(){
                        self.addtoggle=false;
                    }
            }

            if ui.button("save").clicked(){
                for page in &self.pages{
                std::fs::write(format!("{}.md", &page.title),Page::markdown(&page));
                }
            }
        }
    
    
            );



        egui::CentralPanel::default().show(ui, |ui| {
            let current_page=&mut self.pages[self.page_idx];

            //ui.heading(&current_page.title);
            ui.add(egui::TextEdit::singleline(&mut current_page.title).font(egui::FontId::proportional(24.0))
                .desired_width(f32::INFINITY)
                .frame(egui::Frame::NONE));
            ui.separator();
            
            let mut add_new_block:usize =0;
            for (index,block) in &mut current_page.blocks.iter_mut().enumerate(){


                let fontStyle = match block.block_type{
                    BlockType::Heading1=>egui::FontId::proportional(24.0),
                    BlockType::Heading2=>egui::FontId::proportional(20.0),
                    BlockType::Heading3=>egui::FontId::proportional(18.0),
                    BlockType::Paragraph=>egui::FontId::proportional(14.0),
                };



                let editor=ui.add(egui::TextEdit::multiline(&mut block.content)
                .font(fontStyle)
                    .desired_width(f32::INFINITY)
                    .desired_rows(1)
                    .frame(egui::Frame::NONE.inner_margin(egui::Margin::symmetric(8,4))));

                if block.needs_focus{
                    editor.request_focus();
                    block.needs_focus=false;
                }

                //if block.content.ends_with("\n"){
                if editor.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) && !ui.input(|i| i.modifiers.shift) {
                    add_new_block=index+1;
                    block.content.pop();

                }
                if editor.lost_focus(){
                    block.is_editing=false;
                }

                
                if editor.gained_focus(){
                    block.is_editing=true;
                    
                }

                if editor.hovered()/*||editor.has_focus()*/{
                    ui.painter().rect_stroke(
                        editor.rect,
                        2.0,
                        ui.visuals().widgets.hovered.bg_stroke,
                        egui::StrokeKind::Inside,
                    );
                }
                if editor.gained_focus(){
                if block.block_type==BlockType::Heading1{
                        block.content.insert_str(0,"# ");
                }else if block.block_type==BlockType::Heading2{
                    block.content.insert_str(0,"## ");
                }else if block.block_type==BlockType::Heading3{
                    block.content.insert_str(0,"### ");
                }
            }

                //let is_active= block.is_editing; //|| block.content_is_empty();
                if block.is_editing{
                    

                if block.content.starts_with("# "){
                    block.block_type=BlockType::Heading1;
                } else if block.content.starts_with("## "){
                    block.block_type=BlockType::Heading2
                }else if block.content.starts_with("### "){
                    block.block_type=BlockType::Heading3
                }else {
                    block.block_type=BlockType::Paragraph
                }
            }

            
            
            if editor.lost_focus(){
                if block.content.starts_with("# "){
                    block.content.replace_range(0..2, "");
                } else if block.content.starts_with("## "){
                    block.content.replace_range(0..3, "");
                }else if block.content.starts_with("### "){
                    block.content.replace_range(0..4, "");
                
            }}}


            

            
            if (add_new_block!=0){
                current_page.addblock(Block::new(String::from("")),add_new_block);
            }

            ui.separator();

            
            
        });

        
    }
}


fn main() -> eframe::Result<()>{

    let mut pagevec=Vec::new();
    let mut untitledpage=Page::new(String::from("Untitled"));
    let dummyblock=Block::new(String::from(""));
    untitledpage.addblock(dummyblock,0);
    pagevec.push(untitledpage);

    let esnote=EsNote{
        pages:pagevec,
        page_idx:0,
        new_page_name:String::new(),
        addtoggle:false,
    };

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "EsNote",
        native_options,
        Box::new(|_cc| Ok(Box::new(esnote)))
    )


}

