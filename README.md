# HenHacks2024 : Team CowSay

## Team Members

- Alex Barb
  - Writing Discord bot commands
- Heather Cassel
  - Writing Discord bot commands
  - Video Creation
- Ben Crocker
  - Scaffolding the project
  - Connecting the Discord bot with Klipper through UNIX domain sockets
- Mace Scovell
  - Setting up 3D Printer
  - Configuring Klipper

## Project Description

CowSay is a Discord bot that allows users to remotely control a 3D printer.

The bot is written in Rust and uses Serenity to interact with Discord.
The bot then uses UNIX domain sockets to communicate with Klipper, a 3D printer firmware.

The bot manages a print queue for the printer, and allows users to upload and manage gcode files.

Ideally an operator will be at the 3D printer watching it as it prints, and the bot will allow them to monitor and control the printer from their phone or computer or anything that has Discord.

## Project Goals

We want to create an accessible Discord bot for people to request prints and monitor their progress.
A Discord bot was chosen as the interface because it is a widely used platform and is accessible from many devices, making
it easy for any user regardless of their technical background to use the bot.
