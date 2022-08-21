use lib_simulation::Observer;

pub struct GenerationObserver {
    id: String,
    generation: usize,
}

impl GenerationObserver {
    pub fn new(id: String) -> Self {
        Self { id, generation: 0 }
    }

    fn update_page(&self) {
        let element = get_element(&self.id);
        element.set_inner_html(&format!("{}", self.get()));
    }
}

impl Observer<usize> for GenerationObserver {
    fn set(&mut self, t: usize) -> bool {
        self.generation = t;
        self.update_page();
        true
    }

    fn get(&self) -> usize {
        self.generation
    }
}

/// Structure to handle dynamic changes for generation and
/// the list of Fitnesses over time.
pub struct FitnessObserver {
    id: String,
    fitnesses: Vec<f32>,
    generation_observer: GenerationObserver,
}

impl FitnessObserver {
    pub fn new(id: String, generation_observer: GenerationObserver) -> Self {
        Self {
            id,
            fitnesses: vec![],
            generation_observer,
        }
    }

    pub fn update_page(&self) {
        get_element(&self.id).set_inner_html(&format!("{:05.3}", self.get()));
    }
}

impl Observer<f32> for FitnessObserver {
    fn get(&self) -> f32 {
        self.fitnesses.last().unwrap_or(&0.0).clone()
    }

    fn set(&mut self, t: f32) -> bool {
        self.fitnesses.push(t);
        self.update_page();
        self.generation_observer.set(self.fitnesses.len());
        self.fitnesses.last().unwrap() == &t
    }
}

fn get_element(id: &str) -> web_sys::Element {
    web_sys::window()
        .expect("global `window` should exist")
        .document()
        .expect("should have a `document` on `window`")
        .get_element_by_id(id)
        .unwrap()
}
