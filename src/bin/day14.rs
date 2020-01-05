use std::collections::{HashMap, VecDeque};
use std::convert::TryFrom;

static FUEL: &'static str = "FUEL";

#[derive(Clone, Debug)]
struct Component {
    name: String,
    quantity: usize,
}

impl Component {
    fn new(name: String, quantity: usize) -> Self {
        Component { name, quantity }
    }
}

#[derive(Clone, Debug)]
struct Formula {
    requirements: Vec<Component>,
    result: Component,
}


impl TryFrom<&str> for Formula {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<Vec<Vec<&str>>> = value
            .split("=>")
            .map(|x| x.trim())
            .map(|side| {
                side.split(",")
                    .map(|component| component.trim())
                    .map(|component| component.split(" ").collect())
                    .collect()
            })
            .collect();
        if parts.len() != 2 {
            return Err(format!("Expect requirements and result: {}", value));
        }
        if parts[1].len() != 1 && parts[1][0].len() != 2 {
            return Err(format!(
                "Expect result to be formed of single part: {:?}",
                parts[1]
            ));
        }
        let result = Component::new(parts[1][0][1].to_owned(), parts[1][0][0].parse().unwrap());
        let requirements = parts[0]
            .iter()
            .map(|component| Component::new(component[1].to_owned(), component[0].parse().unwrap()))
            .collect();

        Ok(Formula {
            requirements,
            result,
        })
    }
}

fn required_batches(amount_required: usize, amount_per_batch: usize) -> usize {
    let additional = match amount_required % amount_per_batch {
        0 => 0,
        _ => 1,
    };
    (amount_required / amount_per_batch) + additional
}

#[derive(Debug)]
struct RecipeGraph {
    formulas: HashMap<String, Formula>,
    extra_resources: HashMap<String, usize>,
}

impl RecipeGraph {
    fn required_ore_for_single_fuel(&mut self) -> Option<usize> {
        self.require_ore_for_fuel(1)
    }

    fn require_ore_for_fuel(&mut self, count: usize) -> Option<usize> {
        let mut to_calc = VecDeque::with_capacity(self.formulas.len());
        to_calc.push_front((FUEL, count));
        let mut required_ore = 0;

        while let Some(next) = to_calc.pop_front() {
            match next {
                ("ORE", amount) => {
                    required_ore += amount;
                }
                (id, mut amount) => {
                    let available_excess = self.extra_resources.entry(id.to_owned()).or_insert(0);
                    let used_excess = std::cmp::min(*available_excess, amount);
                    *available_excess -= used_excess;
                    amount -= used_excess;
                    if amount > 0 {
                        let num_batches = required_batches(
                            amount,
                            self.formulas.get(id).unwrap().result.quantity,
                        );
                        let formula = self.formulas.get(id).unwrap();
                        *available_excess += (num_batches * formula.result.quantity) - amount;
                        for sub_compound in &formula.requirements {
                            to_calc.push_back((
                                &sub_compound.name,
                                sub_compound.quantity * num_batches,
                            ));
                        }
                    }
                }
            }
        }
        Some(required_ore)
    }
}

impl TryFrom<&str> for RecipeGraph {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let formulas = value
            .lines()
            .map(|line| Formula::try_from(line).unwrap())
            .map(|formula| (formula.result.name.clone(), formula))
            .collect();
        Ok(RecipeGraph { formulas, extra_resources: HashMap::new() })
    }
}

fn main() {
    let mut g = RecipeGraph::try_from(
        "5 HLJD, 1 QHSZD, 13 SKZX => 8 MQPH
        10 LSLV => 4 JNJHW
        1 MQGF, 4 ZWXDQ, 1 GNSZ => 9 DGDH
        1 SKZX, 3 DJSP => 1 MCHV
        6 TWSR, 10 ZHDFS, 10 LQZXQ => 9 LXQNX
        1 FRVW, 1 CJTW => 9 BRCB
        20 ZHVNP => 8 XMXL
        7 JQJXP => 1 ZGZDW
        13 KRCM => 6 KXPQ
        4 ZWXDQ, 4 KFKQF, 1 DZDX => 2 MQGF
        8 DZDX, 2 ZKGM => 3 KFKQF
        3 FXFTB => 8 KVDGP
        10 MVGLF, 3 MWFBW, 13 XMXL, 1 CJTW, 2 ZSXJZ, 2 TNCZH, 3 MPFKN, 6 LXQNX => 2 MZMZQ
        5 FRVW => 3 NWBTP
        1 MVGLF, 2 NLXD, 6 KVDGP, 2 MQPH, 4 FXTJ, 10 TKXKF, 2 FRWV => 2 CSNS
        13 TWSR => 9 BNWT
        2 KRCM => 7 LSLV
        1 ZHDFS, 11 NTVZD, 1 JQJXP => 6 ZHVNP
        2 MCHV, 1 JNJHW => 6 NDQNH
        32 SMHJH, 6 KXPQ => 1 CJTW
        15 FXFTB, 1 MVGLF => 9 MPFKN
        119 ORE => 9 KRCM
        3 TNCZH => 9 BFQLT
        5 MPFKN, 7 TKXKF, 6 JQJXP, 2 DZDX, 16 LCQJ, 4 DGDH, 4 ZGZDW => 7 WVXW
        1 ZHDFS, 1 LXQNX => 3 TNCZH
        4 ZMVKM, 1 BRQT => 3 QHSZD
        24 FRVW, 1 KVDGP, 2 ZLNM => 3 FGLNK
        2 KXPQ, 1 LSLV, 22 HNRQ => 5 ZWXDQ
        6 ZWXDQ => 1 FRVW
        1 FXFTB, 2 MWFBW => 6 ZHDFS
        32 FRVW => 5 FRWV
        6 FXFTB, 6 NDQNH, 2 MWFBW => 1 JQJXP
        9 ZMVKM, 6 QHSZD, 5 LSLV => 4 SMHJH
        3 CHKZ => 6 HLJD
        21 BFQLT => 6 FXTJ
        1 SMHJH, 4 FXFTB => 6 CHKZ
        13 FRVW, 13 JQJXP, 1 GNSZ => 8 ZSXJZ
        2 NDQNH => 8 NTVZD
        3 KRCM => 2 ZKGM
        13 ZHDFS, 14 ZWXDQ, 1 CHKZ => 7 LQZXQ
        2 BNWT, 3 CHKZ => 7 ZLNM
        167 ORE => 1 BRQT
        1 LSLV => 3 DZDX
        8 MZMZQ, 7 NWBTP, 3 WVXW, 44 MQPH, 3 DJSP, 1 CSNS, 3 BRCB, 32 LQZXQ => 1 FUEL
        8 ZLNM => 2 NLXD
        30 JQJXP, 9 FGLNK => 7 LCQJ
        1 ZKGM, 19 KXPQ => 8 DJSP
        4 DJSP => 6 FXFTB
        25 NFTPZ => 6 ZMVKM
        14 ZHVNP, 1 MVGLF => 9 TKXKF
        1 BRQT => 2 SKZX
        6 ZKGM => 7 HNRQ
        3 DZDX => 5 TWSR
        1 SMHJH => 7 MVGLF
        3 NDQNH => 1 GNSZ
        153 ORE => 9 NFTPZ
        14 MCHV, 4 JNJHW, 2 DJSP => 4 MWFBW"
    )
    .expect("failed to create graph");
    println!("{:?}", g.required_ore_for_single_fuel());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut g = RecipeGraph::try_from(
            "10 ORE => 10 A
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL",
        )
        .expect("failed to create graph");
        assert_eq!(g.required_ore_for_single_fuel(), Some(31));
    }

    #[test]
    fn example2() {
        let mut g = RecipeGraph::try_from(
            "9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL",
        )
        .expect("failed to create graph");
        assert_eq!(g.required_ore_for_single_fuel(), Some(165));
    }

    #[test]
    fn example3() {
        let mut g = RecipeGraph::try_from(
            "157 ORE => 5 NZVS
            165 ORE => 6 DCFZ
            44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
            12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
            179 ORE => 7 PSHF
            177 ORE => 5 HKGWZ
            7 DCFZ, 7 PSHF => 2 XJWVT
            165 ORE => 2 GPVTF
            3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        )
        .expect("failed to create graph");
        assert_eq!(g.required_ore_for_single_fuel(), Some(13_312));
    }

    #[test]
    fn example4() {
        let mut g = RecipeGraph::try_from(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
            17 NVRVD, 3 JNWZP => 8 VPVL
            53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
            22 VJHF, 37 MNCFX => 5 FWMGM
            139 ORE => 4 NVRVD
            144 ORE => 7 JNWZP
            5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
            5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
            145 ORE => 6 MNCFX
            1 NVRVD => 8 CXFTF
            1 VJHF, 6 MNCFX => 4 RFSQX
            176 ORE => 6 VJHF",
        )
        .expect("failed to create graph");
        assert_eq!(g.required_ore_for_single_fuel(), Some(180_697));
    }
}
