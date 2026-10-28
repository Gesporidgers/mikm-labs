use csv::WriterBuilder;
use serde::Serialize;

fn v(a: f64, b: f64, V: f64,P: f64) -> f64 {
    (a - b * P) * V
}

fn p(c: f64, d: f64, V: f64, P: f64) -> f64 {
    (-c + d * V) * P
}

#[derive(Copy, Clone)]
#[derive(Serialize)]
struct Row {
    t: f64,
    V: f64,
    P: f64,

    #[serde(skip)]
    k1: f64,
    #[serde(skip)]
    k2: f64,
    #[serde(skip)]
    k3: f64,
    #[serde(skip)]
    k4: f64,

    #[serde(skip)]
    l1: f64,
    #[serde(skip)]
    l2: f64,
    #[serde(skip)]
    l3: f64,
    #[serde(skip)]
    l4: f64,
}

impl Row {
    pub fn kn(&self) -> f64 {
        self.k1 + 2. * self.k2 + 2. * self.k3 + self.k4
    }

    pub fn ln(&self) -> f64 {
        self.l1 + 2. * self.l2 + 2. * self.l3 + self.l4
    }

    pub fn new() -> Self {
        Self {
            t: 0.,
            V: 0.,
            P: 0.,

            k1: 0.,
            k2: 0.,
            k3: 0.,
            k4: 0.,

            l1: 0.,
            l2: 0.,
            l3: 0.,
            l4: 0.,
        }
    }
}

fn main() {
    const T: f64 = 60.;
    const H: f64 = 0.01;

    let a: f64 = 1.;
    let b: f64 = 0.1;
    let c: f64 = 0.85;
    let d: f64 = 0.08;

    const STEPS: usize = ((T / H) + 1.) as usize;

    let mut t_cur: f64 = 0.;

    let mut table: [Row; STEPS] = [Row::new(); STEPS];
    table[0].V = 22.;
    table[0].P = 11.;
    for i in 0..table.len()-1 {
        table[i].t = t_cur;

        table[i].k1 = v(a, b, table[i].V, table[i].P);
        table[i].l1 = p(c, d, table[i].V, table[i].P);

        table[i].k2 = v(a, b,table[i].V + (H / 2.) * table[i].k1, table[i].P + (H / 2.) * table[i].l1);
        table[i].l2 = p(c, d,table[i].V + (H / 2.) * table[i].k1, table[i].P + H / 2. * table[i].l2);

        table[i].k3 = v(a, b,table[i].V + (H / 2.) * table[i].k2, table[i].P + (H / 2.) * table[i].l2);
        table[i].l3 = p(c, d,table[i].V + (H / 2.) * table[i].k2, table[i].P + (H / 2.) * table[i].l2);

        table[i].k4 = v(a, b,table[i].V + H * table[i].k3, table[i].P + H * table[i].l3);
        table[i].l4 = p(c, d,table[i].V + H * table[i].k3, table[i].P + H * table[i].k3);
        

        table[i + 1].V = table[i].V + H * (1. / 6.) * table[i].kn();
        table[i + 1].P = table[i].P + H * (1. / 6.) * table[i].ln();

        t_cur += H;
    }

    let mut wtr = WriterBuilder::new()
                            .has_headers(true)
                            .from_path("out_lotka.csv").unwrap();

    for row in table {
        let _ = wtr.serialize(row);
        let _ = wtr.flush();
    }
}