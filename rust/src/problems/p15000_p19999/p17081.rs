/// Author: juyoung35

use std::io::{self, prelude::*, BufWriter};
use std::fmt;
type Pointer = usize;
type OrnamentBit = u8;
const NONE: OrnamentBit = 0b0;
const HR: OrnamentBit = 0b_0000_0001;
const RE: OrnamentBit = 0b_0000_0010;
const CO: OrnamentBit = 0b_0000_0100;
const EX: OrnamentBit = 0b_0000_1000;
const DX: OrnamentBit = 0b_0001_0000;
const HU: OrnamentBit = 0b_0010_0000;
const CU: OrnamentBit = 0b_0100_0000;
#[derive(Clone, Copy, PartialEq, Eq)]
enum Ornament {
    None,
    HpRegeneration,
    Reincarnation,
    Courage,
    Experience,
    Dexterity,
    Hunter,
    Cursed,
}
impl Default for Ornament {
    fn default() -> Self { Self::None }
}
impl Ornament {
    const HR: Self = Self::HpRegeneration;
    const RE: Self = Self::Reincarnation;
    const CO: Self = Self::Courage;
    const EX: Self = Self::Experience;
    const DX: Self = Self::Dexterity;
    const HU: Self = Self::Hunter;
    const CU: Self = Self::Cursed;
    fn modify(&mut self, input: &str) {
        *self = match input {
            "HR" => Self::HR,
            "RE" => Self::RE,
            "CO" => Self::CO,
            "EX" => Self::EX,
            "DX" => Self::DX,
            "HU" => Self::HU,
            "CU" => Self::CU,
            _ => unreachable!(),
        }
    }
}
#[derive(Clone, Copy, Default)]
struct Weapon(usize);
#[derive(Clone, Copy, Default)]
struct Armor(usize);
#[derive(Clone, Copy)]
enum Item {
    None,
    Weapon(Weapon),
    Armor(Armor),
    Ornament(Ornament),
}
impl Default for Item {
    fn default() -> Self { Self::None }
}
impl Item {
    fn set(&mut self, c: char) {
        *self = match c {
            'W' => Self::Weapon(Weapon::default()),
            'A' => Self::Armor(Armor::default()),
            'O' => Self::Ornament(Ornament::default()),
            _ => unreachable!(),
        };
    }
    fn modify(&mut self, c: char, input: &str) {
        self.set(c);
        match c {
            'W' => if let Self::Weapon(weapon) = self {
                *weapon = Weapon(input.parse::<usize>().unwrap());
            },
            'A' => if let Self::Armor(armor) = self {
                *armor = Armor(input.parse::<usize>().unwrap());
            },
            'O' => if let Self::Ornament(ornament) = self {
                ornament.modify(input);
            },
            _ => unreachable!(),
        }
    }
    fn interact(&self, player: &mut Player) {
        match self {
            &Self::Weapon(weapon) => player.weapon = weapon,
            &Self::Armor(armor) => player.armor = armor,
            &Self::Ornament(ornament) => player.new_ornament(ornament),
            _ => unreachable!(),
        }
    }
}
#[derive(Clone, Copy, Default)]
struct Monster {
    name: Pointer,
    atk: usize,
    def: usize,
    max_hp: usize,
    hp: usize,
    xp: usize,
}
impl Monster {
    fn modify(&mut self, name: Pointer, atk: usize, def: usize, hp: usize, xp: usize) {
        *self = Self { name, atk, def, max_hp: hp, hp, xp, };
    }
    fn ready_combat(&mut self) {
        self.hp = self.max_hp;
    }
    fn first_attack(&self, player: &mut Player) -> GameEvent {
        player.get_first_attacked(self.atk)
    }
    fn attack(&self, player: &mut Player) -> GameEvent {
        player.get_attacked(self.atk)
    }
    fn get_attacked(&mut self, dmg: usize) -> GameEvent {
        self.get_damage(umx(1, uss(dmg, self.def)))
    }
    fn get_damage(&mut self, dmg: usize) -> GameEvent {
        if dmg >= self.hp { return self.death() }
        self.hp -= dmg;
        GameEvent::InProcess
    }
    fn death(&mut self) -> GameEvent { 
        GameEvent::Win
    }
}
#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Wall,
    Box(Item),
    Spike,
    Monster(Monster),
    Boss(Monster),
}
impl Default for Cell {
    fn default() -> Self { Self::Empty }
}
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Empty => '.',
            Self::Wall => '#',
            Self::Box(_) => 'B',
            Self::Spike => '^',
            Self::Monster(_) => '&',
            Self::Boss(_) => 'M',
        })
    }
}
impl Cell {
    fn modify_monster(&mut self, name: Pointer, atk: usize, def: usize, hp: usize, xp: usize) {
        if let Self::Monster(monster) | Self::Boss(monster) = self {
            monster.modify(name, atk, def, hp, xp);
        }
    }
    fn modify_box(&mut self, c: char, input: &str) {
        if let Self::Box(item) = self { 
            item.modify(c, input);
        }
    }
    fn interact(&mut self, player: &mut Player) -> GameEvent {
        match self {
            Self::Empty => (),
            Self::Box(item) => {
                item.interact(player);
                *self = Self::Empty;
            },
            Self::Spike => match player.step_on_spike() {
                GameEvent::Death => return GameEvent::KilledBySpike,
                e => return e,
            },
            Self::Monster(monster) => {
                monster.ready_combat();
                match player.first_attack(monster) {
                    GameEvent::InProcess => (),
                    GameEvent::Win => {
                        player.win(monster.xp);
                        *self = Self::Empty;
                        return GameEvent::InProcess;
                    },
                    e => return e,
                }
                loop {
                    match monster.attack(player) {
                        GameEvent::InProcess => (),
                        GameEvent::Death => return GameEvent::KilledByMonster(monster.name),
                        e => return e,
                    }
                    match player.attack(monster) {
                        GameEvent::InProcess => (),
                        GameEvent::Win => {
                            player.win(monster.xp);
                            *self = Self::Empty;
                            return GameEvent::InProcess;
                        },
                        e => return e,
                    }
                }
            },
            Self::Boss(monster) => {
                player.ready_boss_combat();
                monster.ready_combat();
                match player.first_attack(monster) {
                    GameEvent::InProcess => (),
                    GameEvent::Win => {
                        player.win(monster.xp);
                        *self = Self::Empty;
                        return GameEvent::Win;
                    },
                    e => return e,
                }
                match monster.first_attack(player) {
                    GameEvent::InProcess => (),
                    GameEvent::Death => return GameEvent::KilledByMonster(monster.name),
                    e => return e,
                }
                loop {
                    match player.attack(monster) {
                        GameEvent::InProcess => (),
                        GameEvent::Win => {
                            player.win(monster.xp);
                            *self = Self::Empty;
                            return GameEvent::Win;
                        },
                        e => return e,
                    }
                    match monster.attack(player) {
                        GameEvent::InProcess => (),
                        GameEvent::Death => return GameEvent::KilledByMonster(monster.name),
                        e => return e,
                    }
                }
            },
            Self::Wall => unreachable!(),
        }
        GameEvent::InProcess
    }
}
struct Player {
    y: usize,
    x: usize,
    yi: usize,
    xi: usize,
    max_hp: usize,
    hp: usize,
    atk: usize,
    def: usize,
    xp: usize,
    lvl: usize,
    weapon: Weapon,
    armor: Armor,
    ornaments: Vec<Ornament>,
    flag: OrnamentBit,
}
impl Player {
    fn new(y: usize, x: usize) -> Self {
        Self {
            y,
            x,
            yi: y,
            xi: x,
            max_hp: 20,
            hp: 20,
            atk: 2,
            def: 2,
            xp: 0,
            lvl: 1,
            weapon: Weapon::default(),
            armor: Armor::default(),
            ornaments: Vec::default(),
            flag: NONE,
        }
    }
    fn new_ornament(&mut self, ornament: Ornament) {
        if self.ornaments.len() >= 4 { return }
        if self.ornaments.iter().all(|&o| o != ornament) {
            self.ornaments.push(ornament);
            self.flag |= match ornament {
                Ornament::None => NONE,
                Ornament::HR => HR,
                Ornament::RE => RE,
                Ornament::CO => CO,
                Ornament::EX => EX,
                Ornament::DX => DX,
                Ornament::HU => HU,
                Ornament::CU => CU,
            };
        }
    }
    fn step_on_spike(&mut self) -> GameEvent {
        let dmg = if self.flag & DX == DX { 1 } else { 5 };
        self.get_damage(dmg)
    }
    fn ready_boss_combat(&mut self) {
        if self.flag & HU == HU {
            self.hp = self.max_hp;
        }
    }
    fn get_damage_multiplier(&self) -> usize {
        match (self.flag & CO == CO, self.flag & DX == DX) {
            (true, b) => 2 + btu(b),
            _ => 1,
        }
    }
    fn first_attack(&self, monster: &mut Monster) -> GameEvent {
        monster.get_attacked((self.atk + self.weapon.0) * self.get_damage_multiplier())
    }
    fn attack(&self, monster: &mut Monster) -> GameEvent {
        monster.get_attacked(self.atk + self.weapon.0)
    }
    fn get_first_attacked(&mut self, dmg: usize) -> GameEvent {
        if self.flag & HU == HU { return GameEvent::InProcess }
        self.get_attacked(dmg)
    }
    fn get_attacked(&mut self, dmg: usize) -> GameEvent {
        self.get_damage(umx(1, uss(dmg, self.def + self.armor.0)))
    }
    fn get_damage(&mut self, dmg: usize) -> GameEvent {
        if dmg >= self.hp { return self.death() }
        self.hp -= dmg;
        GameEvent::InProcess
    }
    fn death(&mut self) -> GameEvent {
        if self.flag & RE == RE { return self.rebirth() }
        self.hp = 0;
        GameEvent::Death
    }
    fn rebirth(&mut self) -> GameEvent {
        for i in 0..self.ornaments.len() {
            if self.ornaments[i] == Ornament::RE {
                self.ornaments.remove(i);
                break;
            }
        }
        self.flag &= !RE;
        self.hp = self.max_hp;
        self.y = self.yi;
        self.x = self.xi;
        GameEvent::Rebirth
    }
    fn try_level_up(&mut self) {
        if self.xp >= 5 * self.lvl { self.level_up() }
    }
    fn level_up(&mut self) {
        self.xp = 0;
        self.lvl += 1;
        self.max_hp += 5;
        self.hp = self.max_hp;
        self.atk += 2;
        self.def += 2;
    }
    fn win(&mut self, mut xp: usize) {
        if self.flag & HR == HR {
            self.hp = umn(self.hp + 3, self.max_hp);
        }
        if self.flag & EX == EX {
            xp = ((xp as f64) * 1.2) as usize;
        }
        self.xp += xp;
        self.try_level_up();
    }
}
#[derive(PartialEq, Eq)]
enum GameEvent {
    InProcess,
    Win,
    KilledByMonster(Pointer),
    KilledBySpike,
    Death,
    Rebirth,
    ToBeContinued,
}
fn btu(b: bool) -> usize { b as usize }
fn uwa(l: usize, r: usize) -> usize { usize::wrapping_add(l, r) }
fn umx(l: usize, r: usize) -> usize { usize::max(l, r) }
fn umn(l: usize, r: usize) -> usize { usize::min(l, r) }
fn uss(l: usize, r: usize) -> usize { usize::saturating_sub(l, r) }
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut it = input.trim_end().split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>().unwrap();
    let [n, m] = [0; 2].map(|_| read());
    let mut grid = vec![vec![Cell::default(); m]; n];
    let (mut y, mut x) = (0, 0);
    let mut k = 0;
    let mut l = 0;
    for j in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        for (i, c) in input.trim_end().chars().enumerate() {
            grid[j][i] = match c {
                '.' => Cell::Empty,
                '#' => Cell::Wall,
                'B' => {
                    l += 1;
                    Cell::Box(Item::default())
                },
                '^' => Cell::Spike,
                '&' => {
                    k += 1;
                    Cell::Monster(Monster::default())
                },
                'M' => {
                    k += 1;
                    Cell::Boss(Monster::default())
                },
                '@' => {
                    (y, x) = (j, i);
                    continue;
                },
                _ => unreachable!(),
            }
        }
    }
    let mut turns = String::new();
    io::stdin().read_line(&mut turns).unwrap();
    let mut player = Player::new(y, x);
    let mut names = Vec::with_capacity(k);
    for _ in 0..k {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut it = input.trim_end().split_ascii_whitespace();
        let mut next = || it.next().unwrap();
        let [r, c] = [0; 2].map(|_| next().parse::<usize>().unwrap() - 1);
        let name = next().to_owned();
        let idx = names.len();
        names.push(name);
        let [w, a, h, e] = [0; 4].map(|_| next().parse::<usize>().unwrap());
        grid[r][c].modify_monster(idx, w, a, h, e);
    }
    for _ in 0..l {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut it = input.trim_end().split_ascii_whitespace();
        let mut next = || it.next().unwrap();
        let [r, c] = [0; 2].map(|_| next().parse::<usize>().unwrap() - 1);
        match next().chars().next().unwrap() {
            item => grid[r][c].modify_box(item, next()),
        }
    }
    let mut o = BufWriter::new(io::stdout());
    let mut res = GameEvent::ToBeContinued;
    let mut passed_turn = 0;
    for direction in turns.trim_end().chars() {
        passed_turn += 1;
        let (dy, dx) = match direction {
            'L' => (0, !0),
            'R' => (0, 1),
            'U' => (!0, 0),
            'D' => (1, 0),
            _ => unreachable!(),
        };
        let (mut ny, mut nx) = (uwa(player.y, dy), uwa(player.x, dx));
        let e = match grid.get_mut(ny).and_then(|row| row.get_mut(nx)).unwrap_or(&mut Cell::Wall) {
            Cell::Wall => {
                (ny, nx) = (player.y, player.x);
                grid[player.y][player.x].interact(&mut player)
            },
            cell => cell.interact(&mut player),
        };
        match e {
            GameEvent::InProcess => {
                player.y = ny;
                player.x = nx;
            },
            GameEvent::Rebirth => (),
            e => {
                player.y = ny;
                player.x = nx;
                res = e;
                break;
            },
        }
    }
    for y in 0..n {
        for x in 0..m {
            if player.hp > 0 && y == player.y && x == player.x {
                write!(o, "@").unwrap();
                continue;
            }
            write!(o, "{}", grid[y][x]).unwrap();
        }
        writeln!(o).unwrap();
    }
    writeln!(o, "Passed Turns : {}", passed_turn).unwrap();
    writeln!(o, "LV : {}", player.lvl).unwrap();
    writeln!(o, "HP : {}/{}", player.hp, player.max_hp).unwrap();
    writeln!(o, "ATT : {}+{}", player.atk, player.weapon.0).unwrap();
    writeln!(o, "DEF : {}+{}", player.def, player.armor.0).unwrap();
    writeln!(o, "EXP : {}/{}", player.xp, 5 * player.lvl).unwrap();
    match res {
        GameEvent::Win => writeln!(o, "YOU WIN!").unwrap(),
        GameEvent::KilledByMonster(name) => writeln!(o, "YOU HAVE BEEN KILLED BY {}..", names[name]).unwrap(),
        GameEvent::KilledBySpike => writeln!(o, "YOU HAVE BEEN KILLED BY SPIKE TRAP..").unwrap(),
        GameEvent::ToBeContinued => writeln!(o, "Press any key to continue.").unwrap(),
        _ => unreachable!(),
    }
}