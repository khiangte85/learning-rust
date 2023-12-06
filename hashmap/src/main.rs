use std::collections::HashMap;

#[derive(Debug)]
struct Team {
    played: u8,
    win: u8,
    loss: u8,
    draw: u8,
    points: u8,
}

fn main() {
    let mut teams = HashMap::new();

    teams.insert(
        String::from("Arsenal"),
        Team {
            played: 15,
            win: 11,
            loss: 1,
            draw: 3,
            points: 36,
        },
    );
    teams.insert(
        String::from("Liverpool"),
        Team {
            played: 14,
            win: 19,
            loss: 2,
            draw: 4,
            points: 31,
        },
    );

    println!("{:#?}", teams.get(&String::from("Arsenal")).unwrap().played);

    for (key, value) in &teams {
        println!("{key} {:?}", value);
    }

    println!("{:?}", teams);

    let man_city = Team {
        played: 14,
        win: 9,
        draw: 3,
        loss: 2,
        points: 30,
    };

    teams.entry(String::from("Man City")).or_insert(man_city);


    for (key, value) in &teams {
        println!("{key} {:?}", value);
    }


    let sentence = "hello world this is new world";

    let mut map = HashMap::new();

    for word in sentence.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count  +=  1;
    }

    println!("{:?}", map);

}
