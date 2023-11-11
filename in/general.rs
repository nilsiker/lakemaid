struct Car {
    color: Color,
    passenger: Vec<Passenger>,
}

enum Color {
    Red,
    Green,
    Blue
}

enum Seat {
    Driver,
    Shotgun,
    DJ
}

struct Passenger {
    person_info: PersonInfo,
    seat: Seat,
}

struct PersonInfo {
    name: String,
    age: u8,
}
