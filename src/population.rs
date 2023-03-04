use opencv::imgproc::circle;
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
}

struct Population {
    individuals: Vec<Individual>,
}