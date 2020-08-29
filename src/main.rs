use std::collections::HashSet;
use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Type {
    Nature,
    Air,
    Water,
    Light,
    Energy,
    Psi,
    Stone,
    Ice,
    Fire,
    Dark,
    Chaos,
    Metal,
}

impl Type {
    fn effective_against(self) -> &'static [Type] {
        match self {
            Type::Nature => &[Type::Psi, Type::Chaos],
            Type::Air => &[Type::Water, Type::Psi, Type::Chaos, Type::Metal],
            Type::Water => &[Type::Fire, Type::Dark, Type::Metal],
            Type::Light => &[Type::Stone, Type::Fire, Type::Dark, Type::Chaos],
            Type::Energy => &[Type::Water, Type::Psi, Type::Metal],
            Type::Psi => &[Type::Light, Type::Stone, Type::Ice],
            Type::Stone => &[Type::Air, Type::Energy],
            Type::Ice => &[Type::Nature, Type::Air, Type::Water, Type::Energy],
            Type::Fire => &[Type::Nature, Type::Air, Type::Ice, Type::Chaos],
            Type::Dark => &[Type::Air, Type::Psi, Type::Stone, Type::Fire],
            Type::Chaos => &[Type::Water, Type::Psi],
            Type::Metal => &[Type::Nature, Type::Psi, Type::Fire],
        }
    }

    fn ineffective_against(self) -> &'static [Type] {
        match self {
            Type::Nature => &[Type::Ice, Type::Fire, Type::Dark, Type::Metal],
            Type::Air => &[Type::Energy, Type::Stone, Type::Ice, Type::Fire, Type::Dark],
            Type::Water => &[Type::Air, Type::Energy, Type::Ice, Type::Chaos],
            Type::Light => &[Type::Psi, Type::Metal],
            Type::Energy => &[Type::Stone, Type::Ice],
            Type::Psi => &[Type::Nature, Type::Air, Type::Energy, Type::Dark, Type::Chaos, Type::Metal],
            Type::Stone => &[Type::Light, Type::Psi, Type::Dark],
            Type::Ice => &[Type::Psi, Type::Stone, Type::Fire],
            Type::Fire => &[Type::Water, Type::Light, Type::Dark, Type::Metal],
            Type::Dark => &[Type::Water, Type::Light],
            Type::Chaos => &[Type::Nature, Type::Air, Type::Light, Type::Fire],
            Type::Metal => &[Type::Air, Type::Water, Type::Energy, Type::Ice],
        }
    }

    fn resilient_to(self) -> &'static [Type] {
        match self {
            Type::Nature => &[Type::Psi, Type::Chaos],
            Type::Air => &[Type::Water, Type::Psi, Type::Chaos, Type::Metal],
            Type::Water => &[Type::Fire, Type::Dark, Type::Metal],
            Type::Light => &[Type::Stone, Type::Fire, Type::Dark, Type::Chaos],
            Type::Energy => &[Type::Air, Type::Water, Type::Psi, Type::Metal],
            Type::Psi => &[Type::Light, Type::Stone, Type::Ice],
            Type::Stone => &[Type::Air, Type::Energy, Type::Ice],
            Type::Ice => &[Type::Nature, Type::Air, Type::Water, Type::Energy, Type::Metal],
            Type::Fire => &[Type::Nature, Type::Air, Type::Ice, Type::Chaos],
            Type::Dark => &[Type::Nature, Type::Air, Type::Psi, Type::Stone, Type::Fire],
            Type::Chaos => &[Type::Water, Type::Psi],
            Type::Metal => &[Type::Nature, Type::Light, Type::Psi, Type::Fire],
        }
    }

    fn weak_to(self) -> &'static [Type] {
        match self {
            Type::Nature => &[Type::Ice, Type::Fire, Type::Metal],
            Type::Air => &[Type::Stone, Type::Ice, Type::Fire, Type::Dark],
            Type::Water => &[Type::Air, Type::Energy, Type::Ice, Type::Chaos],
            Type::Light => &[Type::Psi],
            Type::Energy => &[Type::Stone, Type::Ice],
            Type::Psi => &[Type::Nature, Type::Air, Type::Energy, Type::Dark, Type::Chaos, Type::Metal],
            Type::Stone => &[Type::Light, Type::Psi, Type::Dark],
            Type::Ice => &[Type::Psi, Type::Fire],
            Type::Fire => &[Type::Water, Type::Light, Type::Dark, Type::Metal],
            Type::Dark => &[Type::Water, Type::Light],
            Type::Chaos => &[Type::Nature, Type::Air, Type::Light, Type::Fire],
            Type::Metal => &[Type::Air, Type::Water, Type::Energy, Type::Ice],
        }
    }
}

fn main() {
    let types = [Type::Nature, Type::Air, Type::Water, Type::Light, Type::Energy, Type::Psi, Type::Stone, Type::Ice, Type::Fire, Type::Dark, Type::Chaos, Type::Metal];
    let mut effective_against_spread: [u64; 13] = [0; 13];
    let mut ineffective_against_spread: [u64; 13] = [0; 13];
    let mut resilient_to_spread: [u64; 13] = [0; 13];
    let mut weak_to_spread: [u64; 13] = [0; 13];
    let mut combo_effective_against = HashSet::new();
    let mut combo_ineffective_against = HashSet::new();
    let mut combo_resilient_to = HashSet::new();
    let mut combo_weak_to = HashSet::new();
    for combination in types.iter().combinations(5) {
        for member in combination.iter() {
            for t in member.effective_against() {
                combo_effective_against.insert(*t);
            }
            for t in member.ineffective_against() {
                combo_ineffective_against.insert(*t);
            }
            for t in member.resilient_to() {
                combo_resilient_to.insert(*t);
            }
            for t in member.weak_to() {
                combo_weak_to.insert(*t);
            }
        }
        effective_against_spread[combo_effective_against.len()] += 1;
        ineffective_against_spread[combo_ineffective_against.len()] += 1;
        resilient_to_spread[combo_resilient_to.len()] += 1;
        weak_to_spread[combo_weak_to.len()] += 1;
        if combo_effective_against.len() == 7 {
            println!("{:?} - Weaknesses: {}", combination, combo_weak_to.len());
        }
        if combo_weak_to.len() == 6 {
            //println!("{:?} - Effective against: {} Weaknesses: {}", combination, combo_effective_against.len(), combo_weak_to.len());
        }
        combo_effective_against.clear();
        combo_ineffective_against.clear();
        combo_resilient_to.clear();
        combo_weak_to.clear()
    }
    println!("Effective against spread: {:?}", effective_against_spread);
    println!("Ineffective against spread: {:?}", ineffective_against_spread);
    println!("Resilient to spread: {:?}", resilient_to_spread);
    println!("Weak to spread: {:?}", weak_to_spread);
}
