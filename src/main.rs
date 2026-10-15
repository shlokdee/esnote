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


let first_note=Block{
    block_type: BlockType::Paragraph,
    content: String::from("first note lesgoo"),

};

let first_page=Page{
    title: String::from("epic title"),
    blocks: Vec::new(),
    children: Vec::new(),

};

