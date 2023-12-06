use std::collections::BTreeMap;

fn main() {
    const INPUT: &str = include_str!("./input");
    let seeds = INPUT.lines().nth(0).unwrap().split(": ").nth(1).unwrap();

    let mut seeds = seeds
        .split(" ")
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|x| {
            let from = x[0].parse::<i64>().unwrap();
            let len = x[1].parse::<i64>().unwrap();
            SeedRange {
                from,
                to: from + len,
            }
        })
        .collect::<Vec<SeedRange>>();
    seeds.sort_by(|a, b| a.from.cmp(&b.from));

    let fin_layer = flatten_layers(&layer_from_string(INPUT));

    let result = seeds
        .into_iter()
        .map(|x| fin_layer.get_min_value(x))
        .min()
        .unwrap();

    assert_eq!(result, 59370572);
    println!("Day 5, Task 2: {}", result);
}

fn layer_from_string(input: &str) -> Vec<Layer> {
    let mut layers = Vec::new();
    let mut layer = Layer::new();
    for l in input.lines() {
        if l.is_empty() {
            layers.push(layer);
            layer = Layer::new();
            continue;
        }
        if l.contains(":") {
            continue;
        }
        let parts = l
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        layer.add_range(parts[0], parts[1], parts[2]);
    }
    layers.push(layer);
    let layers: Vec<Layer> = layers.into_iter().filter(|x| !&x.is_empty()).collect();
    //layers.iter().for_each(|x| println!("{:?}", x));
    layers
}

fn flatten_layers(layers: &Vec<Layer>) -> Layer {
    layers
        .iter()
        .fold(Layer::new(), |acc, layer| acc.chain(&layer))
}

struct SeedRange {
    from: i64,
    to: i64,
}

#[derive(Debug)]
struct Layer {
    markers: BTreeMap<i64, i64>,
}

impl Layer {
    fn new() -> Self {
        let mut markers = BTreeMap::new();
        markers.insert(0, 0);
        Layer { markers }
    }

    fn is_empty(&self) -> bool {
        self.markers.len() == 1 && self.markers.iter().next().unwrap().1 == &(0 as i64)
    }

    fn probe_diff(&self, value: i64) -> i64 {
        let dif = self.markers.range(..value + 1).next_back().unwrap();
        *dif.1
    }
    fn map_value(&self, value: i64) -> i64 {
        value + self.probe_diff(value)
    }

    fn add_range(&mut self, to_range: i64, from: i64, length: i64) {
        let diff = to_range - from;
        let to = from + length;
        // Get Last Marker before end of Range
        let default_dif = self.probe_diff(to);
        self.markers.insert(to, default_dif);
        self.markers.insert(from, diff);
    }

    /**
     * Combine two layers
     * The Resulting Layer is equivalent to chaining the two layers together
     */
    fn chain(&self, second: &Layer) -> Self {
        let mut new_markers = BTreeMap::<i64, i64>::new();
        let windows = self.markers.iter().zip(self.markers.iter().skip(1));
        for ((ss, sd), (es, _ed)) in windows {
            new_markers.insert(*ss, second.map_value(self.map_value(*ss)) - *ss);

            let section = (*ss + *sd)..(*es + *sd + 1);
            for m in second.markers.range(section) {
                let new_marker = m.0 - sd;
                let new_diff = m.1 + sd;
                new_markers.insert(new_marker, new_diff);
            }
        }

        let (ss, _) = self.markers.iter().last().unwrap();
        new_markers.insert(*ss, 0);

        for (s, d) in second.markers.range(ss..) {
            new_markers.insert(*s, *d);
        }

        Layer {
            markers: new_markers,
        }
    }

    #[allow(dead_code)]
    fn eq(&self, other: &Layer) -> bool {
        self.markers.eq(&other.markers)
    }
    fn get_min_value(&self, seed_range: SeedRange) -> i64 {
        let mut min = self.map_value(seed_range.from);
        for i in self.markers.range(seed_range.from..seed_range.to+1) {
            let value = self.map_value(*i.0);
            min = min.min(value);
        }
        min
    }
}

#[test]
fn layer_chain_comparison() {
    let mut layer1 = Layer::new();
    layer1.add_range(10, 0, 5);
    assert!(layer1.chain(&Layer::new()).eq(&layer1));

    let mut layer1 = Layer::new();
    let mut layer2 = Layer::new();
    let mut layer_r = Layer::new();
    layer1.add_range(10, 0, 5);
    layer2.add_range(20, 10, 5);
    layer_r.add_range(20, 0, 5);
    layer_r.add_range(20, 10, 5);
    assert!(layer1.chain(&layer2).eq(&layer_r));

    let mut layer1 = Layer::new();
    let mut layer2 = Layer::new();
    let mut layer_r = Layer::new();
    layer1.add_range(110, 100, 5);
    layer2.add_range(120, 108, 5);
    //
    // 1 -> 10 -> 22
    // 2 -> 11 -> 23
    // 3 -> 12 -> 24
    // 4 -> 04 -> 04
    // ...
    // 8 -> 08 -> 20
    // 9 -> 09 -> 21
    // 10 -> 10 -> 22
    // 11 -> 11 -> 23
    // 12 -> 12 -> 24
    // 13 -> 13 -> 13
    // ...
    layer_r.add_range(122, 100, 3);
    layer_r.add_range(113, 103, 2);
    layer_r.add_range(120, 108, 5);
    println!("1: {:?}", layer1);
    println!("2: {:?}", layer2);
    println!("3: {:?}", layer_r);
    println!(" = {:?}", layer1.chain(&layer2));
    assert!(layer1.chain(&layer2).eq(&layer_r));
}

#[test]
fn layer_given_values() {
    let layers = layer_from_string(&include_str!("./inputT"));
    let values = vec![(82, 46), (79, 82), (14, 43), (55, 86), (13, 35)];
    for (value, expected) in values {
        let new = layers.iter().fold(value, |acc, layer| {
            print!(" -> {}", layer.map_value(acc));
            layer.map_value(acc)
        });
        println!(" <=> {}", expected);
        assert_eq!(new, expected);
    }
}

#[test]
fn layer_chained_input() {
    let layers = layer_from_string(&include_str!("./inputT"));
    let values = vec![(82, 46), (79, 82), (14, 43), (55, 86), (13, 35)];
    let fin_layer = flatten_layers(&layers);
    for (v, e) in values {
        assert_eq!(fin_layer.map_value(v), e);
    }
}

#[test]
fn layer_chain2() {
    let mut layer1 = Layer::new();
    layer1.add_range(10, 0, 4);
    let mut layer2 = Layer::new();
    layer2.add_range(20, 11, 2);
    let layer3 = layer1.chain(&layer2);
    println!("1: {:?}", layer1);
    println!("2: {:?}", layer2);
    println!("3: {:?}", layer3);

    for i in 0..30 {
        println!(
            "{} -> {} -> {} = {}",
            i,
            layer1.map_value(i),
            layer2.map_value(layer1.map_value(i)),
            layer3.map_value(i)
        );
        assert_eq!(layer3.map_value(i), layer2.map_value(layer1.map_value(i)));
    }
    assert_eq!(layer3.map_value(0), 10);
    assert_eq!(layer3.map_value(1), 20);
    assert_eq!(layer3.map_value(2), 21);
    assert_eq!(layer3.map_value(3), 13);

    assert_eq!(layer3.map_value(4), 4);
    assert_eq!(layer3.map_value(5), 5);
}

#[test]
fn layer_back_and_forth() {
    let mut layer1 = Layer::new();
    layer1.add_range(10, 4, 2);
    let mut layer2 = Layer::new();
    layer2.add_range(4, 10, 2);
    let layer3 = layer1.chain(&layer2);
    println!("1: {:?}", layer1);
    println!("2: {:?}", layer2);
    println!("3: {:?}", layer3);

    for i in 0..30 {
        println!(
            "{} -> {} -> {} = {}",
            i,
            layer1.map_value(i),
            layer2.map_value(layer1.map_value(i)),
            layer3.map_value(i)
        );
        assert_eq!(layer3.map_value(i), layer2.map_value(layer1.map_value(i)));
    }

    assert_eq!(layer3.map_value(0), 0);
    assert_eq!(layer3.map_value(1), 1);
    assert_eq!(layer3.map_value(2), 2);
    assert_eq!(layer3.map_value(3), 3);
    assert_eq!(layer3.map_value(4), 4);
    assert_eq!(layer3.map_value(5), 5);
}

#[test]
fn layer_default() {
    let layer = Layer::new();
    assert_eq!(layer.map_value(0), 0);
    assert_eq!(layer.map_value(1), 1);
    assert_eq!(layer.map_value(2), 2);
    assert_eq!(layer.map_value(3), 3);
    assert_eq!(layer.map_value(4), 4);
    assert_eq!(layer.map_value(5), 5);
}

#[test]
fn layer_start_range() {
    let mut layer = Layer::new();
    layer.add_range(12, 0, 2);
    assert_eq!(layer.map_value(0), 12);
    assert_eq!(layer.map_value(1), 13);

    assert_eq!(layer.map_value(2), 2);
    assert_eq!(layer.map_value(3), 3);
    assert_eq!(layer.map_value(4), 4);
    assert_eq!(layer.map_value(5), 5);
}

#[test]
fn layer_center_range() {
    let mut layer = Layer::new();
    layer.add_range(10, 1, 2);
    assert_eq!(layer.map_value(0), 0);

    assert_eq!(layer.map_value(1), 10);
    assert_eq!(layer.map_value(2), 11);

    assert_eq!(layer.map_value(3), 3);
    assert_eq!(layer.map_value(4), 4);
    assert_eq!(layer.map_value(5), 5);
}

#[test]
fn layer_end_range() {
    let mut layer = Layer::new();
    layer.add_range(12, 4, 2);
    assert_eq!(layer.map_value(0), 0);
    assert_eq!(layer.map_value(1), 1);
    assert_eq!(layer.map_value(2), 2);
    assert_eq!(layer.map_value(3), 3);

    assert_eq!(layer.map_value(4), 12);
    assert_eq!(layer.map_value(5), 13);
}
