use std::alloc::dealloc;
use std::io::Error;
use std::iter::Zip;
use std::ops::Deref;
use std::path::Iter;
use opencv::imgproc::circle;
use opencv::photo::seamless_clone;
use opencv::platform_types::size_t;
use rand::Rng;
use rand::seq::SliceRandom;
use crate::circle::Circle;


pub struct Individual {
    circles: Vec<Circle>,
}

impl Individual {
    fn size(&self) -> usize {
        self.circles.len()
    }

    pub fn merge(individual1: &Individual, individual2: &Individual) -> Individual {
        assert_eq!(individual1.size(), individual2.size(), "cannot merge individuals with different sizes");

        let circles: Vec<Circle> = (0..individual1.size()).map(|i| {
            if rand::random() {
                individual1.circles[i]
            } else {
                individual2.circles[i]
            }
        }).collect();

        Self { circles }
    }

    pub fn random(num_circles: i32, width: u16, height: u16) -> Individual {
        let circles = (0..num_circles)
            .map(|_| Circle::random(width, height))
            .collect();
        Self { circles }
    }

    pub fn draw(&self, mat: &mut opencv::prelude::Mat) {
        self.circles.iter().for_each(|circle| circle.draw(mat));
    }

    pub fn get_mse(&self, target: &opencv::prelude::Mat) -> f64 {
        let mut self_drawn = opencv::prelude::Mat::default();
        self.draw(&mut self_drawn);

        let mut diff = opencv::prelude::Mat::default();
        match opencv::core::absdiff(target, &mut self_drawn, &mut diff) {
            Err(err) => panic!("Can't calculate absdiff: {:?}", err),
            _ => ()
        };

        let mut diff_squared = opencv::prelude::Mat::default();
        match opencv::core::multiply(&diff, &diff, &mut diff_squared, 1., -1) {
            Err(err) => panic!("Can't multiply matrices?! {:?}", err),
            _ => ()
        };

        let scalar = opencv::core::sum_elems(&diff_squared).unwrap();
        scalar[0] + scalar[1] + scalar[2] + scalar[3]
    }
}

struct Population {
    individuals: Vec<Individual>,
}

impl Population {
    pub fn random(num_individuals: i16, num_circles: i32, width: u16, height: u16) -> Population {
        let individuals = (0..num_individuals)
            .map(|_| Individual::random(num_circles, width, height))
            .collect();
        Self { individuals }
    }

    // TODO: configurable loss fn?
    pub fn sort_by_mse(&mut self, target: opencv::prelude::Mat) {
        let mut scores: Vec<f64> = self.individuals.iter().map(|i| i.get_mse(&target)).collect();
        let mut scores_and_individuals: Vec<(f64, &Individual)> = std::iter::zip(scores, &self.individuals).collect();
        scores_and_individuals.sort_unstable_by(|(score_a, _), (score_b, _)| score_a.partial_cmp(score_b).unwrap());
    }

    pub fn next_gen(&mut self, mate_percent: f32) {
        use rand::seq::SliceRandom;
        let cutoff = (mate_percent * (self.individuals.len() as f32)) as usize;
        let top = &self.individuals[..cutoff];
        let mut rng = rand::thread_rng();
        let new_population = (0..self.individuals.len())
            .map(|_| (top.choose(&mut rng), top.choose(&mut rng)))
            .map(|(ind1, ind2)| Individual::merge(ind1.unwrap(), ind2.unwrap()))
            .collect::<Vec<_>>();

        self.individuals = new_population;

        // TODO: important - make configurable
        if rng.gen_range(0..100) == 50 {
            self.mutate_randomly();
        }
    }
    fn mutate_randomly(&self) {
        todo!("maybe use choose_multiple? maybe not")
    }

}

