#!/usr/bin/env python3
import socket
import time
import random

def create_fake_adsb_packet(iteration, previous_altitude_raw, target_increment):
    # Define fixed ADS-B field values
    downlink_format = 17      # 5 bits (0-31)
    capability = 5            # 3 bits (0-7)
    icao_address = 0x123456   # 24 bits, e.g., 0x123456

    # Vary altitude: simulate a takeoff by increasing altitude by around target_increment raw units per second.
    # Here, we introduce a slight random variation: sometimes one unit less, sometimes one unit more.
    variation = random.choice([-1, 0, 0, 1])  # More likely to be 0, so average stays near target_increment.
    altitude_increment = max(0, target_increment + variation)
    altitude_raw = previous_altitude_raw + altitude_increment

    # Vary latitude and longitude slightly to simulate horizontal movement.
    # Smaller changes here to reflect typical movement during takeoff.
    latitude_target = 40.0 + (iteration * 0.001)
    longitude_target = -75.0 + (iteration * 0.001)
    
    latitude_raw = int(round((latitude_target + 90) * (1 << 17) / 180))
    longitude_raw = int(round((longitude_target + 180) * (1 << 17) / 360))
    
    # Pack the fields into a 78-bit integer:
    value = 0
    value = (value << 5) | downlink_format
    value = (value << 3) | capability
    value = (value << 24) | icao_address
    value = (value << 12) | altitude_raw
    value = (value << 17) | latitude_raw
    value = (value << 17) | longitude_raw

    # Shift left by 2 bits to fill 80 bits (10 bytes)
    value = value << 2

    packet = value.to_bytes(10, byteorder='big')
    return packet, altitude_raw

def main():
    UDP_IP = "127.0.0.1"
    UDP_PORT = 3000

    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    iteration = 0
    # Start at ground level (0 raw units equals 0 ft). If you want to simulate an airport at a higher altitude, set a different initial value.
    altitude_raw = 0  
    # Set the desired average raw increment.
    # For a climb rate of ~3000 ft/min (50 ft/s) and 25 ft per raw unit, target_increment should be about 2.
    target_increment = 2  
    
    print(f"Sending fake ADS-B packets to {UDP_IP}:{UDP_PORT}...")
    while True:
        packet, altitude_raw = create_fake_adsb_packet(iteration, altitude_raw, target_increment)
        sock.sendto(packet, (UDP_IP, UDP_PORT))
        # Print the current simulated altitude for reference.
        print(f"Iteration {iteration}: altitude_raw = {altitude_raw}, altitude = {altitude_raw * 25} ft")
        iteration += 1
        # Use a consistent 1-second interval (with slight random variation if desired)
        time.sleep(random.uniform(0.9, 1.1))

if __name__ == '__main__':
    main()
