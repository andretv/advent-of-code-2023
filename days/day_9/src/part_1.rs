pub fn solution(input: &str) -> i64 {
    let mut reports: Vec<Report> = input.lines().map(|line| line.into()).collect();
    let mut sum = 0;

    for report in &mut reports {
        report.extrapolate();
        let last_value = report
            .history
            .first()
            .expect("Last prediction")
            .0
            .last()
            .expect("Last prediction number");

        sum += *last_value as i64;
    }

    sum
}

///
/// Report wrapper.
///
struct Report {
    history: Vec<Prediction>,
}

impl Report {
    fn extrapolate(&mut self) {
        let last_prediction = self
            .history
            .last_mut()
            .expect("Last prediction should always exist");
        last_prediction.0.push(0);

        for index in (0..self.history.len() - 1).rev() {
            let prediction = self.history.get(index).expect("");
            let future_prediction = self.history.get(index + 1).expect("");

            let prediction_left = *prediction.0.last().expect("Last prediction number");
            let prediction_bottom = *future_prediction.0.last().expect("Last prediction number");

            self.history
                .get_mut(index)
                .expect("Last prediction")
                .0
                .push(prediction_left + prediction_bottom);
        }
    }
}

impl std::fmt::Debug for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, r"Report {{",)?;
        for prediction in &self.history {
            writeln!(f, "\t{:?}", prediction.0)?;
        }
        writeln!(f, "}}",)?;
        Ok(())
    }
}

impl From<&str> for Report {
    fn from(value: &str) -> Self {
        let mut predictions: Vec<Prediction> = vec![value.into()];

        while predictions
            .iter()
            .all(|prediction| !prediction.is_all_zeros())
        {
            let last_prediction = predictions
                .last()
                .expect("Predictions should always have at least one prediction");

            predictions.push(last_prediction.predict());
        }

        Self {
            history: predictions,
        }
    }
}

///
/// Prediction wrapper
///
#[derive(Debug)]
struct Prediction(Vec<i32>);

impl Prediction {
    fn is_all_zeros(&self) -> bool {
        self.0.iter().all(|number| *number == 0)
    }

    fn predict(&self) -> Self {
        let mut prediction = vec![];

        for index in 1..self.0.len() {
            prediction.push(self.0[index] - self.0[index - 1])
        }

        Self(prediction)
    }
}

impl From<&str> for Prediction {
    fn from(value: &str) -> Self {
        Self(
            value
                .split_whitespace()
                .map(|number| {
                    number
                        .parse::<i32>()
                        .expect("Number should always be parsable")
                })
                .collect(),
        )
    }
}
