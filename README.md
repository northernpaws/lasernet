# Lasernet

This repository contains the software, hardware, and CAD files for retrofitting lasertag.net Falcon systems with an open-source platform. This involves replacing the PCBs in the tagger and vest units with new Zigbee-controlled PCBs, and having a computer with a Zigbee radio to communicate with them and run the game supervisor software. Additional devices, such as tablets, can be used to manage games over a web interface.

This project is currently a work-in-progress until release v1.0.0. More details for how to assemble the system will be added below as it evolves.

[Tagger Schematics (KiCanvas)](https://kicanvas.org/?github=https%3A%2F%2Fgithub.com%2Fnorthernpaws%2Flasernet%2Fblob%2Fmain%2Ftagger%2Fhardware%2Ftagger.kicad_pro)

## Components

 - An existing lasertag.net Falcon system
 - The retrofit PCBs for the taggers and vests
 - A computer to run the game supervisor
 - A Zigbee interface for the supervisor computer

### Game Supervisor/Controller Computer

For this system you need some type of computer to run the Zigbee interface and game controller software that connects to the units and manages their state, health, etc.

This PC replaces the need for all the units in the system to connect back to an Android tablet over Wifi like the stock laserag.net system does, eliminating Wifi instability from the tablet sleeping, being powered off, battery running out, etc.

We particularly like refurbished Lenovo ThinkCentre Tiny's (such as the M900) for their compact size, power management features (such as auto-power on after power loss), multi-display options, ample USB powers, and an ethernet port, Wifi, and Bluetooth package that gives many options for connectivity. Their reburbished prices are also incredibly afforable, with an M900 with an i7 coming in at around $100CAD, and being office PCs their refurbishment condition tends to be excellent.

### Computer to Zigbee Interface

To communicate with the new units you'll need some type of Zigbee radio or relay that can serve as the Zigbee network coordinator, and allow the PC running the management software to communicate with it. 

Initial software support will be for XBee 3 units in API Mode connected over USB serial, and Texas Instruments Z-Stack devices such as the SMLIGHT SLZB-06M.

We personally like the SMLIGHT SLZB-06M as it can be powered with PoE, simplifying running it to a mount ~12ft up the wall by using a single cable without needing to source additional power at the mounting location. The SLZB-06M is also highly configurable, has suitable range for most indoor arenas, all bundled up into a nice small and portable package that's easy to mount.

## Motivation

I was hired early in 2025 by a center to help repair their lasertag.net Falcon system. The more units I repaired, the more I discovered just how poorly designed the electronics and software components where. The failing units are becoming so regular that I cannot reasonably keep up with them, I'm up to fixing about 5-7 a week with common failures that could have be completely avoided with better mechanical and software design and robust testing that lasertag.net should have done.

I was in regular contact with technical support from the company for several weeks, until they stopped responding after I asked them to connect me to someone I could arrange warrenty repairs with. Likewise, the owner of the business was speaking to their sale representative and a marketing director, who both stopped responding when he asked about repairs and replacement parts. 

The electronics are missing essential safety components, such as a battery charge controller, and some use bad batches of the outdated ESP-12-E (ESP8266 MOD) modules that I've had to replace with the newer ESP-12-F modules on several units to solve Wifi connection problems. The mechanical design around the electronics is also full of issues, such as lacking strain relief on some cables ripping JST connectors off the circuit board under certain common failure conditions.

The software is not robust - missing retry and reconnection mechanisms that cause any minor Wifi instability to result in units dropping out of ongoing games, with no mechanism to reconnect them when they're connected again moments later. The software also relies on all of the game units (possible 20-40+ devices depending on the setup!) having a stable connection to a TCP server running on an Android tablet. If the tablet sleeps, then packets start to be dropped or delayed and causes a myriad of game glitches.

These ongoing issues, and more, have caused the center to loose very large party bookings due to a lack of functional units limiting laser tag game sizes, and have left many players frustrated when a unit fails mid-game and they can't participate.

## Roadmap

Since the business has all of these tagger and vest units that have rebust shells and sewing, we've decided as a "Phase 1" to replace all of the PCBs in the taggers and vests with our own, and then connect them to our own set of software for game management. This will give the business full control over how the system runs, where the failure points are, and allow us to set up a system that's easier for the staff to run and manage.

I'm providing the sources here for anyone else who may be having issues. The goal is that we reach a v2 of this repository where we have the PCB sources files, firmware, and software components required all bundled together into an easily usable package. Eventually I may look at selling replacement retrofit kits and guides.


