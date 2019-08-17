// configuration

//TODO: Xbox button -> windows key
//      Start button -> redo
//      back button -> undo

// Neutral will correspond to everything but l_trigger and modifiers being neutral


// navigation (uses r_trigger)
[
    ((ModifierState::Neutral, JoyStickState


// alphabetic/modable (left trigger)
[
    // vowels
    ((JoyStickState::Neutral, RButtonState::Down), ControllerState::ModableReady('a')),
    ((JoyStickState::Neutral, RButtonState::Left), ControllerState::ModableReady('e')),
    ((JoyStickState::Neutral, RButtonState::Up), ControllerState::ModableReady('i')),
    ((JoyStickState::Neutral, RButtonState::Right), ControllerState::ModableReady('o')),
    ((JoyStickState::Neutral, RButtonState::Bumper), ControllerState::ModableReady('u')),
    ((JoyStickState::Neutral, RButtonState::Trigger), ControllerState::ModableReady('y')),

    // non-voiced
    ((JoyStickState::Up(, RButtonState::Down), ControllerState::ModableReady('p')), //fuck do i include a boolean for joystick state?
    ((JoyStickState::Up, RButtonState::Left), ControllerState::ModableReady('t')),
    ((JoyStickState::Up, RButtonState::Up), ControllerState::ModableReady('f')),
    ((JoyStickState::Up, RButtonState::Right), ControllerState::ModableReady('s')),
    ((JoyStickState::Up, RButtonState::Bumper), ControllerState::ModableReady('c')),
    ((JoyStickState::Up, RButtonState::Trigger), ControllerState::ModableReady('k')),

    // voiced
    ((JoyStickState::Down, RButtonState::Down), ControllerState::ModableReady('b')),
    ((JoyStickState::Down, RButtonState::Left), ControllerState::ModableReady('d')),
    ((JoyStickState::Down, RButtonState::Up), ControllerState::ModableReady('v')),
    ((JoyStickState::Down, RButtonState::Right), ControllerState::ModableReady('z')),
    ((JoyStickState::Down, RButtonState::Bumper), ControllerState::ModableReady('g')),
    ((JoyStickState::Down, RButtonState::Trigger), ControllerState::ModableReady('j')),

    // slides
    ((JoyStickState::Right, RButtonState::Down), ControllerState::ModableReady('h')),
    ((JoyStickState::Right, RButtonState::Left), ControllerState::ModableReady('l')),
    ((JoyStickState::Right, RButtonState::Up), ControllerState::ModableReady('r')),
    ((JoyStickState::Right, RButtonState::Right), ControllerState::ModableReady('w')),
    //putting newline here temporarlit (//TODO:)
    ((JoyStickState::Right, RButtonState::Trigger), ControllerState::ModableReady('\n')),

    // odds
    ((JoyStickState::Right, RButtonState::Down), ControllerState::ModableReady('m')),
    ((JoyStickState::Right, RButtonState::Left), ControllerState::ModableReady('n')),
    ((JoyStickState::Right, RButtonState::Up), ControllerState::ModableReady('q')),
    ((JoyStickState::Right, RButtonState::Right), ControllerState::ModableReady('x')),

    // space
    ((JoyStickState::Neutral, RButtonState::Stick), ControllerState::ModableReady(' ')),
];

// numeric / modable (dpad)
[
    // numbers (low)
    ((DPadState::Down, RButtonState::Down), ControllerState::ModableReady('1')),
    ((DPadState::Down, RButtonState::Left), ControllerState::ModableReady('2')),
    ((DPadState::Down, RButtonState::Up), ControllerState::ModableReady('3')),
    ((DPadState::Down, RButtonState::Right), ControllerState::ModableReady('4')),
    ((DPadState::Down, RButtonState::Bumper), ControllerState::ModableReady('5')),
    ((DPadState::Down, RButtonState::Trigger), ControllerState::ModableReady('6')),

    // numbers (high)
    ((DPadState::Up, RButtonState::Down), ControllerState::ModableReady('7')),
    ((DPadState::Up, RButtonState::Left), ControllerState::ModableReady('8')),
    ((DPadState::Up, RButtonState::Up), ControllerState::ModableReady('9')),
    ((DPadState::Up, RButtonState::Right), ControllerState::ModableReady('0')),
];

// exact (non-modable)
[
    // (+/- configuration on the keyboard is dumb so i changed it)
    ((DPadState::Up, RButtonState::Bumper, ModifierState::Neutral), ControllerState::ExactReady('-')),
    ((DPadState::Up, RButtonState::Trigger, ModifierState::Neutral), ControllerState::ExactReady('+')),
    ((DPadState::Up, RButtonState::Bumper, ModifierState::Shift), ControllerState::ExactReady('_')),
    ((DPadState::Up, RButtonState::Trigger, ModifierState::Shift), ControllerState::ExactReady('=')),

    // scoping (note: RButtonState::Up/Down used for macros)
    ((DPadState::Right, RButtonState::Left, ModifierState::Neutral), ControllerState::ExactReady('(')),
    ((DPadState::Right, RButtonState::Right, ModifierState::Neutral), ControllerState::ExactReady(')')),
    ((DPadState::Right, RButtonState::Left, ModifierState::Shift), ControllerState::ExactReady('[')),
    ((DPadState::Right, RButtonState::Right, ModifierState::Shift), ControllerState::ExactReady(']')),
    ((DPadState::Right, RButtonState::Left, ModifierState::Ctrl), ControllerState::ExactReady('{')),
    ((DPadState::Right, RButtonState::Right, ModifierState::Ctrl), ControllerState::ExactReady('}')),
    ((DPadState::Right, RButtonState::Left, ModifierState::Alt), ControllerState::ExactReady('<')),
    ((DPadState::Right, RButtonState::Right, ModifierState::Alt), ControllerState::ExactReady('>')),

    // quotes
    ((DPadState::Right, RButtonState::Bumper, ModifierState::Neutral), ControllerState::ExactReady('\'')),
    ((DPadState::Right, RButtonState::Bumper, ModifierState::Shift), ControllerState::ExactReady('\"')),
    ((DPadState::Right, RButtonState::Bumper, ModifierState::Ctrl), ControllerState::ExactReady('`')),
    ((DPadState::Right, RButtonState::Bumper, ModifierState::Alt), ControllerState::ExactReady('~')),

    // slashes
    ((DPadState::Right, RButtonState::Trigger, ModifierState::Neutral), ControllerState::ExactReady('/')),
    ((DPadState::Right, RButtonState::Trigger, ModifierState::Shift), ControllerState::ExactReady('\\')),
    ((DPadState::Right, RButtonState::Trigger, ModifierState::Ctrl), ControllerState::ExactReady('|')),
    
    // punctuation
    ((DPadState::Left, RButtonState::Down, ModifierState::Neutral), ControllerState::ExactReady('.')),
    ((DPadState::Left, RButtonState::Left, ModifierState::Neutral), ControllerState::ExactReady(',')),
    ((DPadState::Left, RButtonState::Up, ModifierState::Neutral), ControllerState::ExactReady(':')),
    ((DPadState::Left, RButtonState::Right, ModifierState::Neutral), ControllerState::ExactReady(';')),
    ((DPadState::Left, RButtonState::Bumper, ModifierState::Neutral), ControllerState::ExactReady('?')),
    ((DPadState::Left, RButtonState::Trigger, ModifierState::Neutral), ControllerState::ExactReady('!')),

];
