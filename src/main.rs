#![no_std]
#![no_main]

use kartoffel::*;

// Function to check if the surrounding area is empty, represented by a 3x3 grid of characters
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

        // Check if there's an enemy ('@') directly ahead, if so, stab
        if scan[0][1] == '@' {
            arm_wait();
            arm_stab();
            // Send character '@' to the serial output, indicating an attack
            serial_send('@');
        }
        // Check if there's an enemy to the left, if so, turn left
        else if scan[1][0] == '@' {
            motor_wait();
            motor_turn_left();
            // Send '<' to the serial output, indicating a left turn
            serial_send('<');
        }
        // Check if there's an enemy to the right, if so, turn right
        else if scan[1][2] == '@' {
            motor_wait();
            motor_turn_right();
            // Send '>' to the serial output, indicating a right turn
            serial_send('>');
        }

        // Every 27 iterations, make a slight left turn and move forward
        if c % 27 == 0 {
            motor_wait();
            motor_turn_left();
            motor_wait();
            motor_step();
        }

        // If the surroundings are empty, move forward and set the empty flag
        if empty_surround(&scan) {
            motor_wait();
            motor_step();
            // Send 'E' to the serial output, indicating movement in an empty area
            serial_send('E');
            was_empty = true;
        }
        // If the right side is empty, make a right or left turn based on the previous state
        else if scan[1][2] == '.' {
            motor_wait();
            if was_empty {
                // Turn left if the previous state was empty
                motor_turn_left();
                serial_send('<');
                was_empty = false;
            } else {
                // Otherwise, turn right
                motor_turn_right();
                serial_send('>');
            }
            motor_wait();
            motor_step();
        }
        // If directly ahead is empty, move forward
        else if scan[0][1] == '.' {
            motor_wait();
            motor_step();
            // Send '^' to the serial output, indicating forward movement
            serial_send('^');
        }
        // If the left side is empty, turn left and move forward
        else if scan[1][0] == '.' {
            motor_wait();
            motor_turn_left();
            motor_wait();
            motor_step();
            // Send '<' to the serial output, indicating a left turn
            serial_send('<');
        }

        // Increment the counter after each loop iteration
        c += 1;
    }
}
