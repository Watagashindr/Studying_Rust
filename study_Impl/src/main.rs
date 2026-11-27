
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn say_name(&self) {
        println!("I am {}", self.name);
    }
    fn say_age(&self) {
        println!("I am {} years old", self.age);
    }
}

struct Player {
    name: String,
    hp : i32,

}

impl Player {
    //Player構造体にShowStatusメゾッドをつける
    fn show_status(&self) {
        println!("Player_name:{}",self.name);
        println!("Player_hp:{}",self.hp);
    }

    //ヒールメゾッド
    fn heal_hp(&mut self) {
        self.hp += 10;
    }
}


fn main() {
    let p = Person {
        name : String::from("taro"),
        age: 20,
    };

    p.say_name();
    p.say_age();

    let mut player1 = Player {
        name: String::from("takashi"),
        hp: 4545,
    };


    player1.show_status();

    player1.heal_hp();

    player1.show_status();

}
