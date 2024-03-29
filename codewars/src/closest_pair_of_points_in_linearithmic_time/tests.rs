use crate::closest_pair_of_points_in_linearithmic_time::btreemap_sol;
use crate::closest_pair_of_points_in_linearithmic_time::btreemap_sol::closest_pair;
use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

type Points = ((f64, f64), (f64, f64));

fn verify(actual: Points, expected: Points) {
    if actual == expected || (actual.0 == expected.1 && actual.1 == expected.0) {
        assert!(true)
    } else {
        assert_eq!(actual, expected)
    }
}

#[test]
fn sample_tests() {
    verify(
        closest_pair(&[(2.0, 2.0), (6.0, 3.0)]),
        ((2.0, 2.0), (6.0, 3.0)),
    );
    verify(
        closest_pair(&[
            (2.0, 2.0),
            (2.0, 8.0),
            (5.0, 5.0),
            (6.0, 3.0),
            (6.0, 7.0),
            (7.0, 4.0),
            (7.0, 9.0),
        ]),
        ((6.0, 3.0), (7.0, 4.0)),
    );
    verify(
        closest_pair(&[
            (2.0, 2.0),
            (2.0, 8.0),
            (5.0, 5.0),
            (5.0, 5.0),
            (6.0, 3.0),
            (6.0, 7.0),
            (7.0, 4.0),
            (7.0, 9.0),
        ]),
        ((5.0, 5.0), (5.0, 5.0)),
    );
    verify(
        closest_pair(&[
            (0.8998374006766753, 0.043522294993519575),
            (0.8216028049856874, -0.12236318141421343),
            (0.8404749093035422, 0.14150986215607433),
            (0.68412054221526, -0.05960435780294515),
            (0.9072899317749884, -0.1213320990315292),
            (0.8680109777054663, -0.011004586025835816),
            (0.955564922497625, -0.015255502666314591),
            (0.9095182633279054, -0.0472114610598077),
            (0.7429155700062877, 0.06450600858898675),
            (0.7985128898953212, 0.09487536896197707),
            (0.9967678049453048, -0.04311446994516366),
            (0.9498340450342942, -0.08221834308291104),
            (0.7703868887962588, -0.06844547178125177),
            (0.6865986612274764, 0.022998899629499955),
            (0.8079205484329129, -0.2025073420691922),
            (0.7840591810846355, 0.004583313102512865),
            (0.7688851770861618, -0.1470419009761582),
            (0.8184812921141641, -0.03722683106304919),
            (0.8597824873927521, -0.10281969003377661),
            (0.8628729397138784, 0.09725904195528501),
            (0.6601486780399023, -0.02337303278844652),
            (0.8402555703314102, -0.171823914407645),
            (0.7236405605138294, -0.09801689452850854),
            (0.8296038385996926, 0.04667935706242876),
            (0.7376120836945961, 0.0685205074962138),
            (0.7527932749652894, -0.030321327742144577),
        ]),
        (
            (0.7376120836945961, 0.0685205074962138),
            (0.7429155700062877, 0.06450600858898675),
        ),
    )
}

#[test]
fn performance_test() {
    let points = get_points_from_file("test_data.txt");
    let pair = closest_pair(&points);
    dbg!(&pair);
}

#[test]
fn performance_rnd_test() {
    let mut rng = thread_rng();
    let points = (0..800_000)
        .map(|index| (rng.gen::<f64>(), rng.gen::<f64>()))
        .collect_vec();

    let pair = closest_pair(&points);
    dbg!(&pair);
}

fn get_points_from_file(file: &str) -> Vec<(f64, f64)> {
    //let current_dir = env::current_dir().unwrap();
    let mut test_file =
        File::open(Path::new("src/closest_pair_of_points_in_linearithmic_time").join(file))
            .unwrap();
    let mut test_data = String::new();
    test_file.read_to_string(&mut test_data).unwrap();
    test_data = test_data.strip_prefix("[(").unwrap().to_string();
    test_data = test_data.strip_suffix(",),]").unwrap().to_string();
    test_data
        .split(",),(")
        .map(|str_tuple| {
            str_tuple
                .split(',')
                .map(|f_str| {
                    f_str.parse::<f64>().expect(&*format!(
                        "Invalid number!\nnum: {f_str}\ntuple: {str_tuple}\n"
                    ))
                })
                .collect_tuple::<(f64, f64)>()
                .unwrap()
        })
        .collect_vec()
}
