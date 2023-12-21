use std::{collections::HashMap, str::FromStr};
use strum_macros::EnumString;

#[derive(Debug, EnumString, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    #[strum(serialize = "x")]
    XtremelyCoolLooking,

    #[strum(serialize = "m")]
    Musical,

    #[strum(serialize = "a")]
    Aerodynamic,

    #[strum(serialize = "s")]
    Shiny,
}

pub struct Part {
    ratings: HashMap<Category, usize>,
}

impl Part {
    pub fn rating(&self, category: &Category) -> usize {
        self.ratings.get(category).unwrap().clone()
    }

    pub fn sum_of_ratings(&self) -> usize {
        self.ratings
            .iter()
            .fold(0, |total, (_, value)| total + value)
    }
}

impl From<String> for Part {
    fn from(value: String) -> Self {
        let mut ratings: HashMap<Category, usize> = HashMap::with_capacity(4);

        for rating_str in value
            .strip_prefix("{")
            .unwrap()
            .strip_suffix("}")
            .unwrap()
            .split(",")
            .into_iter()
        {
            let (category, value) = rating_str.split_once("=").unwrap();
            ratings.insert(
                Category::from_str(category).unwrap(),
                value.parse().unwrap(),
            );
        }

        Part { ratings }
    }
}

pub struct Workflow {
    id: String,
    steps: Vec<Step>,
    default: StepResult,
}

impl Workflow {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn result(&self, part: &Part) -> &StepResult {
        for step in &self.steps {
            if step.satisfies(part) {
                return step.result();
            }
        }

        &self.default
    }
}

impl From<String> for Workflow {
    fn from(value: String) -> Self {
        let mut workflow_parts = value.split('{');

        let id = workflow_parts.next().unwrap().to_string();
        let mut steps: Vec<Step> = Vec::new();
        let mut default: Option<StepResult> = None;

        let steps_iter = workflow_parts
            .next()
            .unwrap()
            .strip_suffix("}")
            .unwrap()
            .split(",")
            .into_iter();

        for step in steps_iter {
            match step.contains(":") {
                true => steps.push(Step::from(step)),
                false => default = Some(StepResult::from(step)),
            };
        }

        Workflow {
            id,
            steps,
            default: default.unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StepResult {
    NextStep(String),
    Accepted,
    Rejected,
}

impl From<&str> for StepResult {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            step_id => Self::NextStep(step_id.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Step {
    category: Category,
    comparator: Comparator,
    value: usize,
    result: StepResult,
}

impl Step {
    fn satisfies(&self, part: &Part) -> bool {
        let rating = part.rating(&self.category);

        self.comparator.compare(rating, self.value)
    }

    fn result(&self) -> &StepResult {
        &self.result
    }
}

impl From<&str> for Step {
    fn from(s: &str) -> Self {
        let category = Category::from_str(s.get(0..1).unwrap())
            .expect(format!("Invalid category for step {}", s).as_str());

        let comparator = Comparator::from_str(s.get(1..2).unwrap())
            .expect(format!("Invalid comparator for step {}", s).as_str());

        let (value, next_step) = s.get(2..).unwrap().split_once(":").unwrap();

        Step {
            category,
            comparator,
            value: value.parse().unwrap(),
            result: StepResult::from(next_step),
        }
    }
}

#[derive(Debug, EnumString, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Comparator {
    #[strum(serialize = "<")]
    LessThan,

    #[strum(serialize = ">")]
    GreaterThan,
}

impl Comparator {
    pub fn compare(&self, lhs: usize, rhs: usize) -> bool {
        match self {
            Comparator::LessThan => lhs < rhs,
            Comparator::GreaterThan => lhs > rhs,
        }
    }
}
