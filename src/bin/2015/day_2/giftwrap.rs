use std::io;

fn surface_area(l:u32, w:u32, h:u32) -> u32 {
    2 * ((l * w) + (w * h) + (h * l))
}

fn main() {
    let mut total_giftwrap = 0;
    let mut total_ribbon = 0;
    let mut dimensions_str = String::with_capacity(16);
    while io::stdin().read_line(&mut dimensions_str).is_ok_and(|x| x != 0) {
        let mut lwh: Vec<u32> = dimensions_str.rsplit('x')
            .take(3)
            .map(|val| val.trim().parse::<u32>().unwrap())
            .collect();

        // sort to make finding smallest face easier
        lwh.sort();

        // giftwrap for a present is surface area + area of smallest face
        let surface_area = surface_area(lwh[0], lwh[1], lwh[2]);
        total_giftwrap += surface_area + (lwh[0] * lwh[1]);

        // ribbon for a present is perimeter of the smallest face + the box's volume
        total_ribbon += 2 * (lwh[0] + lwh[1])
        + lwh[0] * lwh[1] * lwh[2];

        dimensions_str.clear();
    }

    println!("We need {total_giftwrap} sq. ft. of wrapping paper RIGHTNOW");
    println!("Also {total_ribbon} ft of ribbon!!");
}
