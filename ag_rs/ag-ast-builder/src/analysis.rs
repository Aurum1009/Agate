pub type LumpError = Result<(), Vec<String>>;

pub trait Validate
where
    Self: Sized,
{
    /// Runs all validation checks, and
    fn validate(&self) -> LumpError {
        let mut errors = vec![];

        {
            let type_res = self.validate_types();
            let var_res = self.validate_variables();
            let call_res = self.validate_calls();

            if let Err(mut errs) = type_res {
                errors.append(&mut errs);
            }
            if let Err(mut errs) = var_res {
                errors.append(&mut errs);
            }
            if let Err(mut errs) = call_res {
                errors.append(&mut errs);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn validate_types(&self) -> LumpError;
    fn validate_variables(&self) -> LumpError;
    fn validate_calls(&self) -> LumpError;
}

pub trait TypeCheck
where
    Self: Sized,
{
    fn type_check(&self, known_types: Vec<String>) -> LumpError;
}
