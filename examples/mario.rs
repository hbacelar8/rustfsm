use rustfsm::{rustfsm, StateBehavior};

#[derive(Clone, Copy, PartialEq)]
enum MarioConsumables {
    Mushroom,
    Flower,
    Feather,
}

#[derive(Clone, Copy, PartialEq)]
enum AliveStates {
    SmallMario,
    BigMario(BigMarioStates),
}

#[derive(Clone, Copy, PartialEq)]
#[allow(clippy::enum_variant_names)]
enum BigMarioStates {
    SuperMario,
    CapeMario,
    FireMario,
}

#[derive(Clone, Copy, PartialEq)]
enum States {
    AliveMario(AliveStates),
    DeadMario,
}

#[derive(Clone, Copy, PartialEq)]
enum Events {
    GetConsumable(MarioConsumables),
    HitMonster,
}

#[derive(Default)]
struct Context {
    number_of_coins: u16,
}

rustfsm!(Mario, States, Events, Context);

impl StateBehavior for States {
    type State = States;
    type Event = Events;
    type Context = Context;

    fn enter(&self, context: &mut Self::Context) {
        match self {
            States::AliveMario(AliveStates::BigMario(BigMarioStates::SuperMario)) => {
                context.number_of_coins += 100
            }

            States::AliveMario(AliveStates::BigMario(BigMarioStates::FireMario)) => {
                context.number_of_coins += 200
            }

            States::AliveMario(AliveStates::BigMario(BigMarioStates::CapeMario)) => {
                context.number_of_coins += 300
            }

            _ => (),
        }
    }

    fn handle(&self, event: &Self::Event, context: &mut Self::Context) -> Option<Self::State> {
        use AliveStates::*;
        use BigMarioStates::*;
        use Events::*;
        use MarioConsumables::*;
        use States::*;

        match (self, event) {
            (AliveMario(SmallMario), GetConsumable(Mushroom)) => {
                Some(AliveMario(BigMario(SuperMario)))
            }

            (
                AliveMario(SmallMario)
                | AliveMario(BigMario(SuperMario))
                | AliveMario(BigMario(CapeMario)),
                GetConsumable(Flower),
            ) => Some(AliveMario(BigMario(FireMario))),

            (
                AliveMario(SmallMario)
                | AliveMario(BigMario(SuperMario))
                | AliveMario(BigMario(FireMario)),
                GetConsumable(Feather),
            ) => Some(AliveMario(BigMario(CapeMario))),

            (AliveMario(SmallMario), HitMonster) => {
                context.number_of_coins = 0;
                Some(DeadMario)
            }

            (_, HitMonster) => {
                context.number_of_coins = 0;
                Some(AliveMario(SmallMario))
            }

            _ => None,
        }
    }
}

impl Mario {
    fn is_alive(&self) -> bool {
        self.current_state != States::DeadMario
    }

    fn number_of_coins(&self) -> u16 {
        self.context.number_of_coins
    }
}

fn main() {
    let mut mario = Mario::new(States::AliveMario(AliveStates::SmallMario));

    // Get a mushroom
    mario.handle(Events::GetConsumable(MarioConsumables::Mushroom));
    assert!(
        mario.get_current_state()
            == States::AliveMario(AliveStates::BigMario(BigMarioStates::SuperMario)),
    );
    assert_eq!(mario.number_of_coins(), 100);
    assert!(mario.is_alive());

    // Get a hit
    mario.handle(Events::HitMonster);
    assert!(mario.get_current_state() == States::AliveMario(AliveStates::SmallMario));
    assert_eq!(mario.number_of_coins(), 0);
    assert!(mario.is_alive());

    // Get a flower
    mario.handle(Events::GetConsumable(MarioConsumables::Flower));
    assert!(
        mario.get_current_state()
            == States::AliveMario(AliveStates::BigMario(BigMarioStates::FireMario))
    );
    assert_eq!(mario.number_of_coins(), 200);
    assert!(mario.is_alive());

    // Get a feather
    mario.handle(Events::GetConsumable(MarioConsumables::Feather));
    assert!(
        mario.get_current_state()
            == States::AliveMario(AliveStates::BigMario(BigMarioStates::CapeMario))
    );
    assert_eq!(mario.number_of_coins(), 500);
    assert!(mario.is_alive());

    // Get a hit
    mario.handle(Events::HitMonster);
    assert!(mario.get_current_state() == States::AliveMario(AliveStates::SmallMario));
    assert_eq!(mario.number_of_coins(), 0);
    assert!(mario.is_alive());

    // Oh no
    mario.handle(Events::HitMonster);
    assert!(mario.get_current_state() == States::DeadMario);
    assert_eq!(mario.number_of_coins(), 0);
    assert!(!mario.is_alive());
}
