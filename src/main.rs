#![no_std]
#![no_main]

use kartoffel::*;

fn empty_surround(scan: &[[char; 3]; 3]) -> bool {
    for x in scan {
        for y in x {
            if *y != '.' && *y != '@' {
                return false;
            }
        }
    }
    true
}

#[no_mangle]
fn main() {
    let mut was_empty = false;
    let mut c = 0;
    loop {
        radar_wait();
        let scan = radar_scan_3x3();

        if scan[0][1] == '@' {
            arm_wait();
            arm_stab();
            serial_send('@');
        } else if scan[1][0] == '@' {
            motor_wait();
            motor_turn_left();
            serial_send('<');
        } else if scan[1][2] == '@' {
            motor_wait();
            motor_turn_right();
            serial_send('>');
        }

        if c % 27 == 0 {
            motor_wait();
            motor_turn_left();
            motor_wait();
            motor_step();
        }

        if empty_surround(&scan) {
            motor_wait();
            motor_step();
            serial_send('E');
            was_empty = true;
        } else if scan[1][2] == '.' {
            motor_wait();
            if was_empty {
                motor_turn_left();
                serial_send('<');
                was_empty = false;
            } else {
                motor_turn_right();
                serial_send('>');
            }
            motor_wait();
            motor_step();
        } else if scan[0][1] == '.' {
            motor_wait();
            motor_step();
            serial_send('^');
        } else if scan[1][0] == '.' {
            motor_wait();
            motor_turn_left();
            motor_wait();
            motor_step();
            serial_send('<');
        }

        c += 1;
    }
}
