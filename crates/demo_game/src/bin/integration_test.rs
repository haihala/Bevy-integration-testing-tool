use bevy_integration_test_tool::*;
use demo_game::{test_assert, DemoGamePlugin};

fn main() {
    test_scenario!("plain_jumps", test_assert, DemoGamePlugin);
}