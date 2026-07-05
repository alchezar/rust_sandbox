// IKinder

#![allow(
    clippy::explicit_counter_loop,
    clippy::map_clone,
    clippy::needless_lifetimes,
    clippy::redundant_static_lifetimes
)]

// -----------------------------------------------------------------------------
// Lifetimes: Intro
// -----------------------------------------------------------------------------

#[derive(Debug)]
enum FrozenItem {
    IceCube,
}

#[derive(Debug)]
struct Freezer {
    contents: Vec<FrozenItem>,
}

fn place_item(freezer: &mut Freezer, item: FrozenItem) {
    freezer.contents.push(item)
}

pub fn intro1() {
    let mut freezer = Freezer {
        contents: Vec::<FrozenItem>::new(),
    };
    let cube = FrozenItem::IceCube;
    place_item(&mut freezer, cube);
}

enum Part {
    Bolt,
    Panel,
}

struct RobotArm<'a> {
    part: &'a Part,
}

struct AssemblyLine {
    parts: Vec<Part>,
}

pub fn intro2() {
    let line = AssemblyLine {
        parts: vec![Part::Bolt, Part::Panel],
    };
    {
        let arm = RobotArm {
            part: &line.parts[0],
        };
    }
}

struct DataType;

fn name<'a>(arg: &'a DataType) -> &'a DataType {
    arg
}

// -----------------------------------------------------------------------------
// Lifetimes: Demo
// -----------------------------------------------------------------------------

#[derive(Debug)]
struct Cards {
    inner: Vec<IdCard>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum City {
    Kovel,
    Lviv,
    Kyiv,
}

#[derive(Debug)]
struct IdCard {
    name: String,
    age: u8,
    city: City,
}

impl IdCard {
    pub fn new(name: String, age: u8, city: City) -> Self {
        Self { name, age, city }
    }
}

fn new_ids() -> Cards {
    Cards {
        inner: vec![
            IdCard::new("Ivan".to_string(), 18, City::Kovel),
            IdCard::new("John".to_string(), 32, City::Lviv),
            IdCard::new("Johan".to_string(), 50, City::Kyiv),
        ],
    }
}

#[derive(Debug)]
struct YoungPeople<'a> {
    inner: Vec<&'a IdCard>,
}

impl<'a> YoungPeople<'a> {
    fn living_in_kovel(&self) -> Self {
        Self {
            inner: self
                .inner
                .iter()
                .filter(|id| id.city == City::Kovel)
                .map(|id| *id)
                .collect(),
        }
    }
}

pub fn demo() {
    let ids = new_ids();

    let mut young = YoungPeople {
        inner: ids.inner.iter().filter(|id| id.age < 45).collect(),
    };

    println!("All");
    ids.inner.iter().for_each(|id| println!("{:?}", id));

    println!("Young");
    young.inner.iter().for_each(|id| println!("{:?}", id));
    println!("Young in Kovel");
    young
        .living_in_kovel()
        .inner
        .iter()
        .for_each(|id| println!("{:?}", id));
}

// -----------------------------------------------------------------------------
// Lifetimes: Activity 1
// -----------------------------------------------------------------------------

const MOCK_DATA: &'static str = "id, first_name,email,dept,title
1,Teresita, tmatisseo@about.com,Training,Programmer Analyst III
2,Parry,pwitch1@java.com,Engineering, Junior Executive
3, Sadella, sconant2@state.tx.us,Accounting,Assistant Professor
4,Rudolfo,rbennison3@netvibes.com,Sales,Dental Hygienist
5,Shellshock,sbeatty4@dmoz.org,Accounting,Chemical Engineer
6,Ethel,edericot5@wsj.com,Legal,Executive Secretary
7,Sigismundo, sancell6@cpanel.net,Sales,GIS Technical Architect
8,Phil,painscough7@sciencedirect.com,Research and Development,Accountant IV
9,Guthry,gspens8@chron.com,Business Development,Senior Cost Accountant
10,Kellyann,kkirley9@posterous.com,Accounting,Assistant Media Planner
11, Cissy, cempsona@paypal.com,Marketing,Professor
12, Estrella, ecapineerb@delicious.com,Marketing, Financial Advisor
13,Gretta,gkernc@nps.gov,Marketing,Marketing Manager
14,Rosa, rodreaind@prweb.com, Engineering,Automation Specialist I";

#[derive(Debug)]
struct Info<'a> {
    names: Vec<&'a str>,
    titles: Vec<&'a str>,
}

pub fn activity1() {
    let data: Vec<_> = MOCK_DATA.split('\n').skip(1).collect();
    let names: Vec<_> = data
        .iter()
        .filter_map(|line| line.split(',').nth(1))
        .map(|name| name.trim())
        .collect();
    let titles: Vec<_> = data
        .iter()
        .filter_map(|line| line.split(',').nth(4))
        .collect();
    let result = Info { names, titles };
    let data = result.names.iter().zip(result.titles.iter());
    let mut id = 1;
    for (name, title) in data {
        println!("{} Name: {}, Title: {}", id, name, title);
        id += 1;
    }
}

// -----------------------------------------------------------------------------
// Lifetimes: Activity 1
// -----------------------------------------------------------------------------

fn longest<'a>(lhs: &'a str, rhs: &'a str) -> &'a str {
    if lhs.len() > rhs.len() { lhs } else { rhs }
}

pub fn activity2() {
    let short = "hello";
    let long = "This is a long message";
    println!("{}", longest(short, long));
}
