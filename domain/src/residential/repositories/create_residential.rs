pub trait ResidentialRepository {
    fn create_residential(
        &self,
        create_residential: NewResidential,
    ) -> Result<(), diesel::result::Error>;

    fn get_residential_by_name(&self, name: &str) -> Option<CreateResidential>;
}
