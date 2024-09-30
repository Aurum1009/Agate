use crate::repr::ProjectRepr;
use ag_utils::{OptLevel, Profile};

pub struct Optimizer<'repr> {
    repr: &'repr mut ProjectRepr,
    opt_level: OptLevel,
    profile: Profile,
}
impl Optimizer<'_> {
    pub fn new<'repr>(
        repr: &'repr mut ProjectRepr,
        opt_level: OptLevel,
        profile: Profile,
    ) -> Optimizer<'repr> {
        Optimizer {
            repr,
            opt_level,
            profile,
        }
    }
    pub fn optimize(mut self) {
        if self.profile == Profile::Debug {
            return;
        }

        if self.opt_level > OptLevel::None {
            self.opt_low();
        }
        if self.opt_level > OptLevel::Low {
            self.opt_medium();
        }
        if self.opt_level > OptLevel::Medium {
            self.opt_high();
        }
    }
    fn opt_low(&mut self) {}
    fn opt_medium(&mut self) {}
    fn opt_high(&mut self) {}
}
