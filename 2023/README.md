# Advent of Code 2023

This year is my second year participating in the AoC competition and also my second year programming in Rust 🦀. Sadly I'd had to cancel my entry to
the 2022 competition due to health issues, I'd improved a lot in writing rust applications through the whole year. So I'm looking forward to test and
evaluate my Rust skills wih new exercises and clear mind.

## 🎯 Goals

The main goal this year is to complete the challenge and solve all puzzles. I'll try to solve them with as few external libraries  

Additionally, I'd like to improve my testing and error handling skills, want to get familiar with `cargo doc` documentation play a little with the 
tracing library. Maybe I will add some other nice features and goals during the AoC2023. 

## 📆 Solutions Quick Access 2023

| Mon               | Tue               | Wed               | Thu               | Fri                       | Sat               | Sun               |
|-------------------|-------------------|-------------------|-------------------|---------------------------|-------------------|-------------------|
|                   |                   |                   |                   | **1.**<br>[Rust](./day01) | **2.**<br>&nbsp;  | **3.**<br>&nbsp;  |
| **4.**<br>&nbsp;  | **5.**<br>&nbsp;  | **6.**<br>&nbsp;  | **7.**<br>&nbsp;  | **8.**<br>&nbsp;          | **9.**<br>&nbsp;  | **10.**<br>&nbsp; |
| **11.**<br>&nbsp; | **12.**<br>&nbsp; | **13.**<br>&nbsp; | **14.**<br>&nbsp; | **15.**<br>&nbsp;         | **16.**<br>&nbsp; | **17.**<br>&nbsp; |
| **18.**<br>&nbsp; | **19.**<br>&nbsp; | **20.**<br>&nbsp; | **21.**<br>&nbsp; | **22.**<br>&nbsp;         | **23.**<br>&nbsp; | **24.**<br>&nbsp; |
| **25.**<br>&nbsp; |                   |                   |                   |                           |                   |                   |

## ⛓️ Dependencies

To get the best experience you will need to install the following external applications / crates:

1. MSRV 1.64.0
2. cargo-watch: ```cargo install cargo-watch```
3. just: ```cargo install just``` _(Windows users: please pay attention to the shell requirements mentioned in their documentation)_

## ▶ How to run a testcase / a solution

You can list all available commands by just executing the `just` command.

To run the tests to prove the implementation details for the given day, just run ```just test <DAY>```. Please note that you have to replace the 
`<DAY>` placeholder with a two digit representation of the given day, e.g. `just test 06` for puzzle 6.

To find a solution for your puzzle input, paste the input from AoC website to the `input-partX.txt` within the matching day folder, where X is whether
1 or 2. Afterwards run ```just run <DAY> <PART>```, e.g. `just run 08 1` to get the solution for puzzle part 1 of day 8.

## 🗨 Feedback
If you have questions or suggestions to improve my code, feel free to open an issue and provide me your feedback.
I'm programming in Rust just for over a year and will be more than happy to receive any suggestions on my code! 