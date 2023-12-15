pub fn solution(input: &str) -> u32 {
    let input = input
        .lines()
        .next()
        .expect("Input should always have a first line")
        .split(",")
        .collect::<Vec<&str>>();

    let mut boxes: [Box; 256] = [Box { slots: [None; 8] }; 256];
    let mut sum = 0;

    for sequence in input {
        let lens = Lens::from(sequence);
        let box_index = lens.get_box_index();

        match &lens.focal_length {
            Some(_) => boxes[box_index].add_lens(lens),
            None => boxes[box_index].remove_lens(lens),
        };
    }

    for (index, cur_box) in boxes.iter().enumerate() {
        sum += cur_box.get_focusing_power(index);
    }

    sum
}

///
/// Box
///
#[derive(Debug, Copy, Clone)]
struct Box<'a> {
    slots: [Option<Lens<'a>>; 8],
}

impl<'a> Box<'a> {
    fn get_focusing_power(&self, box_index: usize) -> u32 {
        let mut sum = 0;
        let box_index = box_index as u32 + 1;

        for (slot_index, slot) in self.slots.iter().enumerate() {
            match slot {
                None => continue,
                Some(lens) => {
                    sum += box_index
                        * (slot_index as u32 + 1)
                        * (lens
                            .focal_length
                            .expect("Lens should always have a focal length"))
                            as u32
                }
            }
        }

        sum
    }

    fn add_lens(&mut self, lens: Lens<'a>) {
        for slot_index in 0..self.slots.len() {
            match self.slots[slot_index] {
                Some(existing_lens) => {
                    if existing_lens.label != lens.label {
                        continue;
                    }

                    self.slots[slot_index] = Some(lens);
                    break;
                }
                None => {
                    self.slots[slot_index] = Some(lens);
                    break;
                }
            };
        }
    }

    fn remove_lens(&mut self, lens: Lens) {
        if let Some(index) = self.slots.iter().position(|slot| {
            let Some(slot) = slot else {
                return false;
            };

            slot.label == lens.label
        }) {
            self.slots[index] = None;
            self.prune_slots();
        };
    }

    fn prune_slots(&mut self) {
        for index in 0..self.slots.len() {
            match self.slots[index] {
                Some(_) => continue,
                None => {
                    for rest_index in index..self.slots.len() {
                        match self.slots[rest_index] {
                            None => continue,
                            Some(lens) => {
                                self.slots[index] = Some(lens);
                                self.slots[rest_index] = None;
                                break;
                            }
                        };
                    }
                }
            };
        }
    }
}

///
/// Lens
///
#[derive(Debug, Copy, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_length: Option<u8>,
}

impl Lens<'_> {
    fn get_box_index(&self) -> usize {
        let mut sum = 0;
        self.label
            .chars()
            .map(|char| char as u8)
            .for_each(|ascii_value| {
                sum += ascii_value as usize;
                sum *= 17;
                sum %= 256;
            });
        sum
    }
}

impl<'a> From<&'a str> for Lens<'a> {
    fn from(value: &'a str) -> Self {
        if let Some(split) = value.split_once("=") {
            return Self {
                label: split.0,
                focal_length: Some(
                    split
                        .1
                        .parse()
                        .expect("Focal length should always be parsable"),
                ),
            };
        }

        if let Some(split) = value.split_once("-") {
            return Self {
                label: split.0,
                focal_length: None,
            };
        }

        unreachable!()
    }
}
