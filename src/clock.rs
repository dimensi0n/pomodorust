enum State {
    Working,
    Resting
}

pub struct Clock {
    work_time: i16,
    rest_time: i16,
    state: State,
    time_left: i16
}


impl Clock {
    pub fn new(work_time: i16, rest_time: i16) -> Clock {
        Clock{
            work_time: work_time,
            rest_time,
            state: State::Working,
            time_left: work_time
        }
    }

    pub fn decrement(&mut self) {
        macro_rules! decrement_condition {
            ($state:expr, $time:ident) => {
                if self.time_left > 0 {
                    self.time_left -= 1;
                    if self.time_left == 0 {
                        self.state = $state;
                        self.time_left = self.$time;
                    }
                }
            };
        }

        match self.state {
            State::Working => decrement_condition!(State::Resting, rest_time),
            State::Resting => decrement_condition!(State::Working, work_time)
        }
    }

    pub fn show(self) -> String {
        let mut time: i16 = self.time_left;
        let minutes: i16 = time / 60;
        time -= (time / 60)  * 60;
        format!("{}:{}", minutes.to_string(), time.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrement() {
        let mut clock = Clock::new(900, 300);
        clock.decrement();
        let is_working: bool = match clock.state {
            State::Working => true,
            State::Resting => false
        };
        assert_eq!(is_working,true);
        assert_eq!(clock.time_left, 899);
    }

    #[test]
    fn test_change_state() {
        let mut clock = Clock::new(1, 300);
        clock.decrement();
        let is_working: bool = match clock.state {
            State::Working => true,
            State::Resting => false
        };
        assert_eq!(is_working,false);
        assert_eq!(clock.time_left, 300);
    }

    #[test]
    fn test_show_clock() {
        let mut clock = Clock::new(900, 300);
        for _ in 1..65 {
            clock.decrement();
        }
        assert_eq!(clock.show(), "13:56");
    }
}
