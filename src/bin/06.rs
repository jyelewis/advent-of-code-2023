fn main() {
    // part 1
    // Time:        48     98     90     83
    // Distance:   390   1103   1112   1360
    let input1: [Race; 4] = [
        Race {
            duration: 48,
            record_distance: 390,
        },
        Race {
            duration: 98,
            record_distance: 1103,
        },
        Race {
            duration: 90,
            record_distance: 1112,
        },
        Race {
            duration: 83,
            record_distance: 1360,
        },
    ];

    let answer1 = challenge(&input1);

    println!("Day 06, Part 1: {}", answer1);
    assert_eq!(answer1, 4568778);

    // part 2
    // Time:        48989083
    // Distance:   390110311121360
    let input2: [Race; 1] = [Race {
        duration: 48989083,
        record_distance: 390110311121360,
    }];

    let answer2 = challenge(&input2);

    println!("Day 06, Part 2: {}", answer2);
    assert_eq!(answer2, 28973936);
}

fn challenge(races: &[Race]) -> u32 {
    races
        .iter()
        .map(|race| race.ways_to_beat_record())
        .reduce(|a, b| a * b)
        .unwrap()
}

struct Race {
    duration: u32,
    record_distance: u64,
}

impl Race {
    pub fn distance_for_button_hold(&self, button_hold_time: u32) -> u64 {
        let boat_speed = button_hold_time as u64;
        let boat_move_time = self.duration - button_hold_time;

        boat_speed * boat_move_time as u64
    }

    pub fn ways_to_beat_record(&self) -> u32 {
        let mut ways = 0;
        for button_hold_time in 1..self.duration {
            let distance = self.distance_for_button_hold(button_hold_time);
            if distance > self.record_distance {
                ways += 1;
            }
        }
        ways
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
