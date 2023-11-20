struct MerchandiserImpl {
    name: String,
    age: u8,
}

impl MerchandiserImpl {
    fn get_name(&self) -> &String { &self.name }
    fn get_age(&self) -> u8 { self.age }
    fn is_full_age(&self) -> bool { self.age >= 18 }
}


fn test() {
    let merchandiser: MerchandiserImpl = "Ivan".into();
}

impl From<&str> for MerchandiserImpl {
    fn from(value: &str) -> Self {
        Self {
            name: value.to_string(),
            age: 42,
        }
    }
}

// impl<T, U> Into<U> for T
//     where U: From<T>,
// {
//     fn into(self) -> U {
//         U::from(self)
//     }
// }

trait Attack {
    fn attack<T: Hp>(&self, victim: &mut T);
}

impl<U> Attack for U
    where U: Damage
{
    fn attack<T: Hp>(&self, victim: &mut T) {
        victim.suffer(self.damage());
    }
}

trait Damage {
    fn damage(&self) -> u32;
}

trait Hp {
    fn suffer(&mut self, damage: u32);
}

struct Hero;

struct Dummy;

impl Hp for Dummy {
    fn suffer(&mut self, _damage: u32) {}
}

impl Damage for Hero {
    fn damage(&self) -> u32 { 42 }
}

fn test_heroes() {
    let hero = Hero;
    let mut dummy = Dummy;
    hero.attack(&mut dummy);
}


trait Employee {
    fn fire(&self);
}

// fn do_fire(employee: impl Employee) {
//     employee.fire()
// }
//
// fn do_fire<T: Employee>(employee: T) {
//     employee.fire()
// }


fn test_dyn() {
    let list: Vec<Box<dyn Location>> = vec![
        Box::new(Tavern),
        Box::new(Tavern),
        Box::new(Pit),
        Box::new(Tavern),
    ];

    let human = Human;
    let orc = Orc;

    for location in &list {
        location.visit(&human);
        location.visit(&orc);
    }
}

struct Tavern;

struct Pit;

struct Human;
struct Orc;

trait Location {
    fn visit(&self, visitor: &dyn Visitor);
}


impl Location for Tavern {
    fn visit(&self, visitor: &dyn Visitor) {
        visitor.accept_tavern(self);
    }
}

impl Location for Pit {
    fn visit(&self, visitor: &dyn Visitor) {
        visitor.accept_pit(self);
    }
}

trait Visitor {
    fn accept_tavern(&self, location: &Tavern);
    fn accept_pit(&self, location: &Pit);
}



impl Visitor for Human {
    fn accept_tavern(&self, _: &Tavern) { println!("Relax") }
    fn accept_pit(&self, _: &Pit) { println!("Bet") }
}
impl Visitor for Orc {
    fn accept_tavern(&self, _: &Tavern) { println!("Brawl") }
    fn accept_pit(&self, _: &Pit) { println!("Fight") }
}