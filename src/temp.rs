
enum FunctionalControllerState{
    keyboard{shift: bool, alt: bool, ctrl: bool, state: KeyboardState},
    navigator{},
}

enum KeyboardState {
    prepared,  // comes off of prepared once a right button is pressed
    alphabetic(Letter),
    keywordic(Keyword),
    numeric(Number),
}

enum Letter {
    A,B,C,D,E,F,G,H,I,J,K,L,M,
    N,O,P,Q,R,S,T,U,V,W,X,Y,Z,
}
enum Number{
    NUM_1,NUM_2,NUM_3,NUM_4,NUM_5,
    NUM_6,NUM_7,NUM_8,NUM_9,NUM_0,
}
enum Keyword {
    IF,
    ENUM,
    STRUCT,
    FOR,
    WHILE,
    MATCH,
    
}

struct RawControllerState {
    left_triggers: triggers_state,
    right_triggers: triggers_state,
    left_pad: pad_state,
    right_pad: pad_state,
}

impl RawControllerState {
    fn update() {
        //TODO: implement
    }
}

enum triggers_state {
    none,
    bumper,
    trigger,
    both,
}

enum pad_state {
    stick{pressed: bool, position: stick_position},
    button(button_choice),
}

enum stick_position {
    neutral,
    left,
    right,
    up,
    down,
}

enum button_choice {
    left,
    right,
    top,
    bottom,
}


fn main() {
    println!("Hello, world!");
}
