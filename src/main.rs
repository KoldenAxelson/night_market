use rand::{thread_rng, Rng};
// use clipboard::ClipboardContext;
// use clipboard::ClipboardProvider;

fn main() {

    let goods_types = vec![
        "Food and Drugs", 
        "Personal Electronics", 
        "Weapons and Armor", 
        "Cyberware", 
        "Clothing and Fasionware", 
        "Survival Gear"];

    let goods_type = dice(6);

    let goods_amount = dice(10)+1;

    let goods = vec![
        // 0-5
        vec![
            "Canned Goods 10eb (Cheap)",
            "Agent 100eb (Premium)",
            "Medium Pistol 50eb (Costly)",
            "Cybereye 100eb (Premium)",
            "Bag Lady Chic",
            "Anti-Smog Breathing Mask 20eb (Everyday)"
        ],
        // 6-10
        vec![
            "Packaged goods 10eb (Cheap)",
            "Programs or Hardware of 100eb or less",
            "Heavy Pistol or Very Heavy Pistol 100eb (Premium)",
            "Cyberaudio Suite 500eb (Expensive)",
            "Gang Colors",
            "Auto Level Dampening Ear Protectors 1,000eb (Very Expensive)"
        ],
        // 11-15
        vec![
            "Frozen Goods 10eb (Cheap)",
            "Audio Recorder 100eb (Premium)",
            "SMG 100eb (Premium)",
            "Neural Link 500eb (Expensive)",
            "Generic Chic",
            "Binoculars 50eb (Costly)"
        ],
        // 16-20
        vec![
            "Bags of Grain 20eb (Everyday)",
            "Bug Detector 500eb (Expensive)",
            "Heavy SMG 100eb (Premium)",
            "Cyberarm 500eb (Expensive)",
            "Bohemian",
            "Carryall 20eb (Everyday)"
        ],
        // 21-25
        vec![
            "Kibble Pack 10eb (Cheap)",
            "Chemical Analyzer 1,000eb (Very Expensive)",
            "Shotgun 500eb (Expensive)",
            "Cyberleg 100eb (Premium)",
            "Leisurewear",
            "Flashlight 20eb (Everyday)"
        ],
        // 26-30
        vec![
            "Bags of Prepak 20eb (Everyday)",
            "Computer 50eb (Costly)",
            "Assault Rifle 500eb (Expensive)",
            "External Cyberware of exactly 1,000eb",
            "Nomad Leathers",
            "Duct Tape 20eb (Everyday)"
        ],
        // 31-35
        vec![
            "Street Drugs of 20eb or less",
            "Cyberdeck 500eb (Expensive)",
            "Sniper Rifle 500eb (Expensive)",
            "External Cyberware of 500eb or less",
            "Asia Pop",
            "Inflatable Bed & Sleep-bag 20eb (Everyday)"
        ],
        // 36-40
        vec![
            "Poor Quality Alcohol 10eb (Cheap)",
            "Disposable Cell Phone 50eb (Costly)",
            "Bows or Crossbow 100eb (Premium)",
            "Internal Cyberware of exactly 1,000eb",
            "Urban Flash",
            "Lock Picking Set 20eb (Everyday)"
        ],
        // 41-45
        vec![
            "Alcohol 20eb (Everyday)",
            "Electric Guitar or Other Instrument 500eb (Expensive)",
            "Grenade Launcher or Rocket Launcher 500eb (Expensive)",
            "Internal Cyberware of 500eb or less",
            "Businesswear",
            "Handcuffs 50eb (Costly)"
        ],
        // 46-50
        vec![
            "Excellent Quality Alcohol 100eb (Premium)",
            "Programs or Hardware of exactly 500eb",
            "Ammunition of 500eb or less",
            "Cybereye Option of exactly 1,000eb",
            "High Fashion",
            "Medtech Bag 100eb (Premium)"
        ],
        // 51-55
        vec![
            "MRE 10eb (Cheap)",
            "Medscanner 1,000eb (Very Expensive)",
            "A Single Exotic Weapon of GM’s choice",
            "Cybereye Option of 500eb or less",
            "Biomonitor 100eb (Premium)",
            "Tent and Camping Equipment 50eb (Costly)"
        ],
        // 56-60
        vec![
            "Live Chicken 50eb (Costly)",
            "Homing Tracer 500eb (Expensive)",
            "Light Melee Weapon 50eb (Costly)",
            "Cyberaudio Option of exactly 1,000eb",
            "Chemskin 100eb (Premium)",
            "Rope (60m/yds) 20eb (Everyday)"
        ],
        // 61-65
        vec![
            "Live Fish 50eb (Costly)",
            "Radio Communicator 100eb (Premium)",
            "Medium Melee Weapon 50eb (Costly)",
            "Cyberaudio Option of 500eb or less",
            "EMP Threading 10eb (Cheap)",
            "Techtool 100eb (Premium)"
        ],
        // 66-70
        vec![
            "Fresh Fruits 50eb (Costly)",
            "Techscanner 1,000eb (Very Expensive)",
            "Heavy Melee Weapon 100eb (Premium)",
            "Neuralware Option of exactly 1,000eb",
            "Light Tattoo 100eb (Premium)",
            "Personal CarePak 20eb (Everyday)"
        ],
        // 71-75
        vec![
            "Fresh Vegetables 50eb (Costly)",
            "Smart Glasses 500eb (Expensive)",
            "Very Heavy Melee Weapon 100eb (Premium)",
            "Neuralware Option of 500eb or less",
            "Shift Tacts 100eb (Premium)",
            "Radiation Suit 1,000eb (Very Expensive)"
        ],
        // 76-80
        vec![
            "Root Vegetables 20eb (Everyday)",
            "Radar Detector 500eb (Expensive)",
            "Armor of 100eb or less",
            "Cyberlimb Option of exactly 1,000eb",
            "Skinwatch 100eb (Premium)",
            "Road Flare 10eb (Cheap)"
        ],
        // 81-85
        vec![
            "Live Pigs 100eb (Premium)",
            "Scrambler/ Descrambler 500eb (Expensive)",
            "Armor of exactly 500eb",
            "Cyberlimb Option of 500eb or less",
            "Techhair 100eb (Premium)",
            "Grapple Gun 100eb (Premium)"
        ],
        // 86-90
        vec![
            "Exotic Fruits 100eb (Premium)",
            "Radio Scanner/ Music Player 50eb (Costly)",
            "Armor of exactly 1,000eb",
            "Fashionware of GM’s Choice",
            "Generic Chic",
            "Tech Bag 500eb (Expensive)"
        ],
        // 91-95
        vec![
            "Exotic Vegetables 100eb (Premium)",
            "Braindance Viewer 1,000eb (Very Expensive)",
            "Weapon Attachments of 100eb or less",
            "Borgware of GM’s Choice",
            "Leisurewear",
            "Shovel or Axe 50eb (Costly)"
        ],
        // 96-100
        vec![
            "Street Drugs of exactly 50eb",
            "Virtuality Goggles 100eb (Premium)",
            "Weapon Attachments of 500eb or higher",
            "Any Cyberware of GM’s Choice",
            "Gang Colors",
            "Airhypo 50eb (Costly)"
        ]
    ];
 
    let mut goods_store = vec![];
    for _ in 0..goods_amount {
        let mut good = goods[dice(20)][goods_type];
        while goods_store.contains(&good) {
            good = goods[dice(20)][goods_type];
        }
        goods_store.push(good);
    }

    let mut output = "".to_string();
    output.push_str("&{template:default}{{name=**");
    output.push_str(goods_types[goods_type]);
    output.push_str("**}}");
    for good in goods_store.iter() {
        output.push_str("{{");
        output.push_str(good);
        output.push_str("}}");
    }

    println!("");

    match goods_type { 
        0=>print!("\x1b[38;5;21;4;1m"),
        1=>print!("\x1b[38;5;22;4;1m"),
        2=>print!("\x1b[38;5;23;4;1m"),
        3=>print!("\x1b[38;5;24;4;1m"),
        4=>print!("\x1b[38;5;25;4;1m"),
        5=>print!("\x1b[38;5;26;4;1m"),
        _=>{}
    }
    println!("{}",goods_types[goods_type]);
    print!("\x1b[0m");

    for good in goods_store.iter() {
             if good.contains("(Everyday)"      ) {print!("\x1b[38;5;8m");println!("{}",good);  } // Set Grey
        else if good.contains("(Cheap)"         ) {print!("\x1b[38;5;208m");println!("{}",good);} // Set Orange
        else if good.contains("(Costly)"        ) {print!("\x1b[38;5;2m");println!("{}",good);  } // Set Green
        else if good.contains("(Premium)"       ) {print!("\x1b[38;5;6m");println!("{}",good);  } // Set Cyan
        else if good.contains("(Expensive)"     ) {print!("\x1b[38;5;3m");println!("{}",good);  } // Set Gold
        else if good.contains("(Very Expensive)") {print!("\x1b[38;5;5m");println!("{}",good);  } // Set Magenta
        else                                      {print!("\x1b[38;5;7m");println!("{}",good);  } // Set White
    }
    
    println!("");

    // let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    // ctx.set_contents(output.to_string()).unwrap();
}

fn dice(n: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0..n) as usize
}