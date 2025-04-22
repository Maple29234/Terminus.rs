use std::collections::HashMap;
use std::io::{Write, stdin, stdout};
use std::process::exit;

trait CloneableFn: FnMut(&mut Game, &mut Terminal, &mut [&mut Item; 4]) + Send + Sync { //taken from the internet
    fn clone_box(&self) -> Box<dyn CloneableFn>;
}

impl<F> CloneableFn for F //taken from the internet
where
    F: Fn(&mut Game, &mut Terminal, &mut [&mut Item; 4]) + Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn CloneableFn> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn CloneableFn> { //taken from the internet
    fn clone(&self) -> Box<dyn CloneableFn> {
        self.clone_box()
    }
}
struct Game {
    points: f32,
    power: u32,
    max_power: u32,
    power_gain: u32,
    points_modifier: f32,
    points_gain: u32,
    help_string: String,
    bought_vars: HashMap<String, bool>,
}
struct Terminal {
    commands: HashMap<String, Box<dyn CloneableFn>>,
    message: String,
}
impl Terminal {
    fn log(&mut self, message: String) {
        let mut s: String = String::new();
        print!("{message}\n",);
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        self.message = s;
    }
    fn next_command(&mut self, game: &mut Game, item: &mut [&mut Item; 4]) {
        {
            let binding = self.message.to_lowercase();
            let message: Vec<&str> = binding.trim().split(" ").collect();
            if self.commands.contains_key(message[0]) {
                self.commands.clone().get_mut(message[0]).unwrap()(game, self, item);
            } else {
                self.log("not found".to_string());
            }
        }
    }
}

struct Item {
    name: String,
    price: u32,
    func: Box<dyn Fn(&mut f32, &mut u32, &mut u32, &mut u32)>,
}
impl Item {
    fn buy_checker(
        &mut self,
        points: &mut f32,
        bought_vars: &mut HashMap<String, bool>,
        terminal: &mut Terminal,
    ) -> bool {
        if *points < self.price as f32 {
            terminal.log("Not enough points".to_string());
            return false;
        }
        let temp_name = self.name.split(":").collect::<Vec<&str>>()[0].to_lowercase();
        if bought_vars.contains_key(&temp_name) && bought_vars[&temp_name] {
            terminal.log("Already bought".to_string());
            return false;
        }
        *points -= self.price as f32;
        if bought_vars.contains_key(&temp_name) {
            *bought_vars.get_mut(&temp_name).unwrap() = true;
        }
        terminal.log(format!("Bought {}\nTotal points left: {points}", self.name));
        true
    }
    fn buy(
        &mut self,
        points: &mut f32,
        bought_vars: &mut HashMap<String, bool>,
        terminal: &mut Terminal,
        points_modifier: &mut f32,
        points_gain: &mut u32,
        power_gain: &mut u32,
        max_power: &mut u32,
    ) {
        let success: bool = self.buy_checker(points, bought_vars, terminal);
        if success {
            (self.func)(points_modifier, points_gain, power_gain, max_power);
        }
    }
}

fn main() {
    let mut game: Game = Game {
        points : 10.00,
        power : 0,
        max_power : 15,
        power_gain : 1,
        points_modifier : 1.00,
        points_gain : 1,
        help_string : "Help - Brings this up\nTutorial - Brings up a tutorial\nShop - Brings up the shop\nCharge - Increase power\nUpdate - Convert power into points\nBalance - Prints your point balance\nGithub - Shows the github repo link\nCredits - Shows the credits\nDiscord - Gives a link to the terminus discord\nSave - Saves your game. MAKE SURE TO SAVE\nLoad - Loads your most recent save".to_string(),
        bought_vars : HashMap::from(
            [("begin".to_string(), false),
            ("index".to_string(), false),
            ("doctype".to_string(), false),
            ("configyml".to_string(),false)])
    };
    let mut terminal: Terminal = Terminal {
        commands: HashMap::new(),
        message: "".to_string(),
    };
    let mut item_list: [&mut Item; 4] = [
        &mut Item {
            name: "Begin: The Beginning".to_string(),
            price: 5,
            func: Box::new(|_points_modifier, points_gain, _power_gain, _max_power| {
                *points_gain += 10;
            }),
        },
        &mut Item {
            name: "Index: Index.html".to_string(),
            price: 20,
            func: Box::new(|points_modifier, _points_gain, _power_gain, _max_power| {
                *points_modifier += 0.5;
            }),
        },
        &mut Item {
            name: "Doctype: <!DOCTYPE HTML>".to_string(),
            price: 50,
            func: Box::new(|points_modifier, _points_gain, _power_gain, _max_power| {
                *points_modifier += 0.5;
            }),
        },
        &mut Item {
            name: "Configyml: config.yml".to_string(),
            price: 100,
            func: Box::new(|_points_modifier, points_gain, _power_gain, _max_power| {
                *points_gain *= 2;
            }),
        },
    ];

    terminal.commands.insert(
        "exit".to_string(),
        Box::new(|_game, _terminal, _item| {
            exit(0);
        }),
    );
    terminal.commands.insert(
        "help".to_string(),
        Box::new(|game, terminal, _item| {
            terminal.log((*game.help_string).to_string());
        }),
    );
    terminal.commands.insert("tutorial".to_string(), Box::new(|_game,terminal,_item| {
        terminal.log("1. Use 'charge' to gain power\n2. After you've gotten max power (or just whenever) sell your power by using 'update'\n3. Check shop and buy stuff by using 'shop'\n4. Use 'save' to save and 'load' to load your most recent save".to_string());
    }));
    terminal.commands.insert(
        "shop".to_string(),
        Box::new(|game, terminal, item| {
            println!("working..");
            for item in item {
                item.buy(
                    &mut game.points,
                    &mut game.bought_vars,
                    terminal,
                    &mut game.points_modifier,
                    &mut game.points_gain,
                    &mut game.power_gain,
                    &mut game.max_power,
                ); //WIP not done yet
            }
        }),
    );
    terminal.commands.insert(
        "charge".to_string(), 
        Box::new(|game,terminal,_item| {
            game.power += game.power_gain;
            if game.power >= game.max_power {
                game.power = game.max_power;
                return terminal.log(format!("Full charge. \n Battery : {}",game.power))
            }
            terminal.log(format!("Gained {} power\nCurrent battery: {}",game.power_gain,game.power));
        })
    );
    terminal.commands.insert(
        "update".to_string(), 
        Box::new(|game,terminal,_item| {
            //Add logic later
        })
    )
    terminal.log("Welcome to Terminus.py\nYou can type 'help' to see available commands. Type 'tutorial' to see a tutorial. Use 'save' to save and 'load' to load your most recent save".to_string());
    loop {
        terminal.next_command(&mut game, &mut item_list);
    }
}
